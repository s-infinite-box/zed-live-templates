use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    time::Duration,
};

use serde_json::{Value, json};
use tower_lsp::lsp_types::Url;

struct Session {
    child: Child,
    stdin: Option<ChildStdin>,
    messages: Receiver<Value>,
    directory: PathBuf,
    uri: String,
}

impl Session {
    fn start() -> Self {
        let directory = std::env::temp_dir().join(format!("templates-lsp-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let uri = Url::from_file_path(directory.join("note.md"))
            .unwrap()
            .to_string();
        let mut child = Command::new(env!("CARGO_BIN_EXE_server"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let (sender, messages) = mpsc::channel();
        std::thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                let mut length = 0;
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap_or(0) == 0 {
                        return;
                    }
                    if line == "\r\n" {
                        break;
                    }
                    if let Some(value) = line.strip_prefix("Content-Length:") {
                        length = value.trim().parse().unwrap();
                    }
                }
                let mut bytes = vec![0; length];
                if reader.read_exact(&mut bytes).is_err() {
                    return;
                }
                if sender
                    .send(serde_json::from_slice(&bytes).unwrap())
                    .is_err()
                {
                    return;
                }
            }
        });
        Self {
            child,
            stdin: Some(stdin),
            messages,
            directory,
            uri,
        }
    }

    fn send(&mut self, message: Value) {
        let body = serde_json::to_vec(&message).unwrap();
        let stdin = self.stdin.as_mut().unwrap();
        write!(stdin, "Content-Length: {}\r\n\r\n", body.len()).unwrap();
        stdin.write_all(&body).unwrap();
        stdin.flush().unwrap();
    }

    fn request(&mut self, id: i32, method: &str, params: Value) -> Value {
        let mut message = json!({"jsonrpc":"2.0", "id":id, "method":method});
        if !params.is_null() {
            message["params"] = params;
        }
        self.send(message);
        loop {
            let message = self
                .messages
                .recv_timeout(Duration::from_secs(10))
                .expect("LSP response timed out");
            if message.get("id") == Some(&json!(id)) {
                assert!(message.get("error").is_none(), "{message}");
                return message["result"].clone();
            }
        }
    }

    fn completion(&mut self, id: i32) -> Value {
        self.request(
            id,
            "textDocument/completion",
            json!({
                "textDocument":{"uri":self.uri}, "position":{"line":0, "character":7}
            }),
        )
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.directory);
    }
}

#[test]
fn packaged_server_protocol_commands_and_reload() {
    let mut session = Session::start();
    let config = session.directory.join("templates.toml");
    let command = if cfg!(windows) {
        r#"["cmd.exe", "/D", "/C", "echo command"]"#
    } else {
        r#"["sh", "-c", "printf command"]"#
    };
    let source = format!(
        r#"version = 1
[variables.value]
command = {command}
[[templates]]
id = "date"
trigger = "dt"
languages = ["Markdown"]
body = '$date$ $value$$END$'
"#
    );
    fs::write(&config, &source).unwrap();
    let initialized = session.request(
        1,
        "initialize",
        json!({
            "processId":null, "capabilities":{},
            "initializationOptions":{"config_path":config}
        }),
    );
    assert_eq!(
        initialized["serverInfo"]["version"],
        env!("CARGO_PKG_VERSION")
    );
    session.send(json!({"jsonrpc":"2.0", "method":"initialized", "params":{}}));
    session.send(json!({"jsonrpc":"2.0", "method":"textDocument/didOpen", "params":{
        "textDocument":{"uri":session.uri, "languageId":"markdown", "version":1, "text":"记录🙂 dt"}
    }}));
    let completions = session.completion(2);
    let item = &completions[0];
    assert_eq!(item["textEdit"]["range"]["start"]["character"], 5);
    assert_eq!(item["textEdit"]["range"]["end"]["character"], 7);
    assert!(
        item["textEdit"]["newText"]
            .as_str()
            .unwrap()
            .ends_with(if cfg!(windows) {
                "command\r\n$0"
            } else {
                "command$0"
            })
    );
    fs::write(
        &config,
        source.replace("$date$ $value$$END$", "changed$END$"),
    )
    .unwrap();
    assert_eq!(session.completion(3)[0]["textEdit"]["newText"], "changed$0");
    fs::remove_file(&config).unwrap();
    assert_eq!(session.completion(4), Value::Null);
    fs::write(&config, &source).unwrap();
    assert_eq!(session.completion(5)[0]["label"], "dt");
    session.request(6, "shutdown", Value::Null);
    session.send(json!({"jsonrpc":"2.0", "method":"exit"}));
    session.stdin.take();
    for _ in 0..100 {
        if let Some(status) = session.child.try_wait().unwrap() {
            assert!(status.success());
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    panic!("server did not exit after shutdown");
}

#[test]
fn version_does_not_require_home_or_configuration() {
    let output = Command::new(env!("CARGO_BIN_EXE_server"))
        .env_remove("HOME")
        .env_remove("XDG_CONFIG_HOME")
        .env_remove("APPDATA")
        .env_remove("USERPROFILE")
        .arg("--version")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap().trim(),
        format!("live-templates-server {}", env!("CARGO_PKG_VERSION"))
    );
}
