use std::{path::Path, process::Stdio, time::Duration};

use anyhow::{Context, Result, ensure};
use serde_json::Value;
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWriteExt},
    process::Command,
    time::timeout,
};

use crate::config::Variable;

pub async fn execute(variable: &Variable, directory: &Path, ctx: &Value) -> Result<String> {
    let mut input = serde_json::to_vec(ctx)?;
    input.push(b'\n');

    let program = Path::new(&variable.command[0]);
    let program = if program.is_relative() && program.components().count() > 1 {
        directory.join(program)
    } else {
        program.to_path_buf()
    };
    let mut command = Command::new(&program);
    command.args(&variable.command[1..]);
    command.current_dir(
        variable
            .cwd
            .as_ref()
            .map_or_else(|| directory.to_path_buf(), |cwd| directory.join(cwd)),
    );
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("TPL_") {
            command.env_remove(key);
        }
    }
    command.env("TPL_CTX_VERSION", "1");
    command.env(
        "TPL_WEEKDAY",
        ctx["now"]["weekday"].as_u64().unwrap().to_string(),
    );
    for (key, value) in [
        ("TPL_VARIABLE", &ctx["variable"]["name"]),
        ("TPL_TEMPLATE_ID", &ctx["template"]["id"]),
        ("TPL_TRIGGER", &ctx["template"]["trigger"]),
        ("TPL_DATE", &ctx["builtins"]["date"]),
        ("TPL_TIME", &ctx["builtins"]["time"]),
        ("TPL_HOST", &ctx["builtins"]["host"]),
        ("TPL_PROJECT_ROOT", &ctx["project"]["root"]),
        ("TPL_PROJECT_NAME", &ctx["project"]["name"]),
        ("TPL_DOCUMENT_URI", &ctx["document"]["uri"]),
        ("TPL_DOCUMENT_PATH", &ctx["document"]["path"]),
        ("TPL_DOCUMENT_NAME", &ctx["document"]["name"]),
        ("TPL_LANGUAGE", &ctx["document"]["language"]),
    ] {
        if let Some(value) = value.as_str() {
            command.env(key, value);
        }
    }
    #[cfg(unix)]
    command.process_group(0);

    let mut child = command
        .spawn()
        .with_context(|| format!("cannot start {}", program.display()))?;
    #[cfg(unix)]
    let _group = ProcessGroup(child.id().unwrap() as i32);
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let stderr = child.stderr.take().unwrap();

    let run = async {
        let write = async {
            match stdin.write_all(&input).await {
                Ok(()) => {}
                Err(error) if error.kind() == std::io::ErrorKind::BrokenPipe => {}
                Err(error) => return Err(error.into()),
            }
            drop(stdin);
            Ok::<_, anyhow::Error>(())
        };
        let wait = async { Ok::<_, anyhow::Error>(child.wait().await?) };
        let (_, stdout, stderr, status) =
            tokio::try_join!(write, read_stdout(stdout), read_stderr(stderr), wait)?;
        let diagnostic = String::from_utf8_lossy(&stderr);
        ensure!(
            status.success(),
            "command exited with {status}: {diagnostic}"
        );
        if !diagnostic.is_empty() {
            eprintln!(
                "variable {}: {diagnostic}",
                ctx["variable"]["name"].as_str().unwrap()
            );
        }
        Ok::<_, anyhow::Error>(String::from_utf8(stdout).context("command output is not UTF-8")?)
    };
    timeout(Duration::from_millis(variable.timeout_ms), run)
        .await
        .with_context(|| format!("command timed out after {} ms", variable.timeout_ms))?
}

async fn read_stdout(reader: impl AsyncRead + Unpin) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader.take(64 * 1024 + 1).read_to_end(&mut bytes).await?;
    ensure!(bytes.len() <= 64 * 1024, "command stdout exceeds 64 KiB");
    Ok(bytes)
}

async fn read_stderr(mut reader: impl AsyncRead + Unpin) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    (&mut reader)
        .take(16 * 1024)
        .read_to_end(&mut bytes)
        .await?;
    tokio::io::copy(&mut reader, &mut tokio::io::sink()).await?;
    Ok(bytes)
}

#[cfg(unix)]
struct ProcessGroup(i32);

#[cfg(unix)]
impl Drop for ProcessGroup {
    fn drop(&mut self) {
        // 清理超时、取消以及命令退出后仍持有管道的子进程。
        unsafe {
            libc::kill(-self.0, libc::SIGKILL);
        }
    }
}
