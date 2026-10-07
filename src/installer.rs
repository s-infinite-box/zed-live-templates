use std::{fs, path::Path};

use zed_extension_api::{self as zed, Architecture, Os};

const VERSION: &str = env!("CARGO_PKG_VERSION");

struct Release {
    target: &'static str,
    executable: &'static str,
    archive: &'static str,
}

impl Release {
    fn for_platform(os: Os, arch: Architecture) -> zed::Result<Self> {
        let target = match (os, arch) {
            (Os::Linux, Architecture::X8664) => "x86_64-unknown-linux-musl",
            (Os::Linux, Architecture::Aarch64) => "aarch64-unknown-linux-musl",
            (Os::Mac, Architecture::X8664) => "x86_64-apple-darwin",
            (Os::Mac, Architecture::Aarch64) => "aarch64-apple-darwin",
            (Os::Windows, Architecture::X8664) => "x86_64-pc-windows-msvc",
            (Os::Windows, Architecture::Aarch64) => "aarch64-pc-windows-msvc",
            _ => return Err("Live Templates requires an x86_64 or ARM64 platform. Set lsp.templates.binary.path to use a custom server.".into()),
        };
        let windows = os == Os::Windows;
        Ok(Self {
            target,
            executable: if windows { "server.exe" } else { "server" },
            archive: if windows { "zip" } else { "tar.gz" },
        })
    }

    fn url(&self) -> String {
        format!(
            "https://github.com/s-infinite-box/zed-live-templates/releases/download/v{VERSION}/live-templates-server-v{VERSION}-{}.{}",
            self.target, self.archive
        )
    }
}

pub fn install(id: &zed::LanguageServerId) -> zed::Result<String> {
    let (os, arch) = zed::current_platform();
    let release = Release::for_platform(os, arch)?;
    let directory = format!("server-v{VERSION}-{}", release.target);
    let path = cached_install(Path::new(&directory), release.executable, |staging| {
        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );
        let file_type = if os == Os::Windows {
            zed::DownloadedFileType::Zip
        } else {
            zed::DownloadedFileType::GzipTar
        };
        zed::download_file(&release.url(), &staging.to_string_lossy(), file_type)
            .map_err(|error| format!("Cannot download Live Templates v{VERSION}: {error}"))?;
        zed::make_file_executable(&staging.join(release.executable).to_string_lossy())
    })?;
    zed::set_language_server_installation_status(id, &zed::LanguageServerInstallationStatus::None);
    Ok(path)
}

// A completed versioned directory is the cache. Partial downloads stay outside it,
// so a failed extraction can be retried on the next language-server start.
fn cached_install(
    directory: &Path,
    executable: &str,
    download: impl FnOnce(&Path) -> zed::Result<()>,
) -> zed::Result<String> {
    let binary = directory.join(executable);
    if fs::metadata(&binary).is_ok_and(|file| file.is_file() && file.len() > 0) {
        return Ok(binary.to_string_lossy().into_owned());
    }
    let staging = directory.with_extension("downloading");
    let perform = || -> zed::Result<()> {
        if staging.exists() {
            fs::remove_dir_all(&staging).map_err(|error| error.to_string())?;
        }
        download(&staging)?;
        if !fs::metadata(staging.join(executable))
            .is_ok_and(|file| file.is_file() && file.len() > 0)
        {
            return Err(format!("Downloaded archive does not contain {executable}"));
        }
        if directory.exists() {
            fs::remove_dir_all(directory).map_err(|error| error.to_string())?;
        }
        fs::rename(&staging, directory).map_err(|error| error.to_string())
    };
    if let Err(error) = perform() {
        let _ = fs::remove_dir_all(&staging);
        return Err(error);
    }
    Ok(binary.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_assets_and_unsupported_architecture() {
        for os in [Os::Linux, Os::Mac, Os::Windows] {
            for arch in [Architecture::X8664, Architecture::Aarch64] {
                let release = Release::for_platform(os, arch).unwrap();
                assert!(release.url().contains(&format!("/v{VERSION}/")));
                assert!(release.url().contains(release.target));
                assert_eq!(release.executable.ends_with(".exe"), os == Os::Windows);
            }
            assert!(Release::for_platform(os, Architecture::X86).is_err());
        }
    }

    #[test]
    fn failed_download_is_retried_and_completed_cache_needs_no_network() {
        let root = std::env::temp_dir().join(format!("templates-installer-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();
        let directory = root.join("version-target");
        assert!(
            cached_install(&directory, "server", |staging| {
                fs::create_dir(staging).unwrap();
                fs::write(staging.join("server"), "partial").unwrap();
                Err("interrupted".into())
            })
            .is_err()
        );
        assert!(!directory.exists());
        assert!(!directory.with_extension("downloading").exists());
        assert!(
            cached_install(&directory, "server", |staging| {
                fs::create_dir(staging).unwrap();
                Ok(())
            })
            .is_err()
        );
        let path = cached_install(&directory, "server", |staging| {
            fs::create_dir(staging).unwrap();
            fs::write(staging.join("server"), "complete").unwrap();
            Ok(())
        })
        .unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "complete");
        assert_eq!(
            cached_install(&directory, "server", |_| panic!("cache must not download")).unwrap(),
            path
        );
        fs::remove_dir_all(root).unwrap();
    }
}
