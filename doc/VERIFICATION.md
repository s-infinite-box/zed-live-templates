# Verification record

## Automatic server distribution: 2026-10-07

The extension now downloads the precompiled server matching its version and platform, caches it in the extension work directory, and lets Zed start it. An explicit `lsp.templates.binary.path` remains an optional override. Normal registry users need neither Rust nor a server path; custom command interpreters remain user-provided.

The final installer runtime is `83308a51748cc967e8258b51583ad33bdea4f69d`. The native servers are published in [release v0.1.0](https://github.com/s-infinite-box/zed-live-templates/releases/tag/v0.1.0), built from tag source `9020e61bf2389e6c2d1f0db3db1ddfc19b36cdfb`. The later installer change preserves the complete version/platform in the staging directory name and does not change native server code.

| Check | Actual result |
| --- | --- |
| Native tests, release build and archive on Linux x86_64 / ARM64, macOS Intel / ARM64, Windows x86_64 / ARM64 | All six passed with Rust 1.90.0 on native GitHub runners |
| Published release | Six platform archives plus `SHA256SUMS`; all release jobs passed before publication |
| Installer tests | Six asset mappings, unsupported x86, interrupted download cleanup, missing executable rejection, retry and cache reuse passed |
| Native integration tests | LSP initialization, command variable, Chinese/emoji UTF-16 range, cursor marker, configuration reload/removal/restoration, shutdown and version without configuration passed |
| WASM extension | Rust 1.90.0 release build and installer tests passed |
| Official Zed 1.22.0, Linux x86_64, fresh installed WASM and empty server cache | First start downloaded v0.1.0 and started the cached static-musl server; no `binary.path` and no source build |
| Official Zed cache reuse, Linux | A second isolated profile containing the downloaded cache started the server with a deliberately unavailable network proxy; binary hash and modification time stayed unchanged |
| Official Zed advanced override, Linux | Started the custom binary with the configured `--config` argument and environment variable despite an unavailable network proxy; no server download cache was created |
| Downloaded Linux release server | Extended LSP checks passed: `dt`, `todo`, `ctx`, `@dt`, Bash/Python variables, UTF-16, cursor, escaping and configuration reload/removal/restoration |
| Windows 11 x86_64 VM, no Rust installed | Published ZIP checksum, native startup with default `APPDATA` and no `HOME`, LSP initialization, command variable, UTF-16 range, reload and shutdown passed |

Evidence: [final runtime CI](https://github.com/s-infinite-box/zed-live-templates/actions/runs/37604193627), [release CI](https://github.com/s-infinite-box/zed-live-templates/actions/runs/37603834223). The Linux installed WASM SHA-256 was `fa683cea1a6c9c9478def48a35a47c389186db0b935ec2a3a012606ff35360c7`. The Windows release ZIP SHA-256 was `bec95265cc43c1b90ed692da7630530da0cf4e9b4c4392f42dca2cf968d9fd4c`.

The new official Linux checks verify installation and startup, and are separate from the maintainer's historical manual expansion tests. The maintainer then entered `dt` in the new official Linux instance; visual inspection confirmed two expansions to `AUTO 2026/10/7 inft`, matching the configured date/hostname template. The maintainer also confirmed the cursor was at the end, after the hostname and trailing space, as specified by `$END$`. This verifies the new installer flow and two `dt` expansions, without implying a new manual run of every historical case. The Mac SSH connection was refused during this run; the six-target CI verifies both Mac native servers, but Mac automatic-download UI testing is pending. Windows has native protocol evidence, not a Zed GUI test. Direct GitHub download from the Windows VM was reset; the same published ZIP and checksum were downloaded on Linux and transferred over SSH for native testing.

On Windows, command timeouts terminate the immediate process; descendant-process cleanup is currently guaranteed only on Unix. The extension continues to register Markdown only. Built-in templates require no Bash or Python; only user-selected command variables require their chosen interpreter.

The prepared registry entry must use the updated public source containing the installer and new documentation. Its exact gitlink is recorded in the registry branch and proposed PR body. The following records describe the earlier preparation and do not establish manual testing of the new installer.

## Historical preparation record (2026-10-06)

## Manual checks reported by the maintainer

On 2026-10-06, the maintainer explicitly confirmed successful manual functional checks in the official Zed distribution on Linux x86_64 and macOS ARM64. These official-Zed results are the primary compatibility evidence. Earlier ZedG logs and environment inspections are supplementary evidence.

| Platform | Reported result | Confirmation date | Editor | Tested commit |
| --- | --- | --- | --- | --- |
| Linux x86_64 | Renamed development extension: manual functional checks passed, confirmed by the maintainer | 2026-10-06 | Official Zed 1.22.0 | `bd55a66` |
| macOS ARM64 | Renamed development extension: manual functional checks passed, confirmed by the maintainer | 2026-10-06 | Official Zed 1.22.0 | `bd55a66` |
| Windows | Not verified; support is not claimed | — | — | — |

The current official-Zed checks on both platforms used Zed 1.22.0 and extension source `bd55a66`, with ID `live-templates-lsp`. The results and setup evidence are documented in the [platform test records](TESTING.md), also available in [Chinese](TESTING.zh-CN.md). Earlier ZedG records identify macOS 26.6.2, ARM64, Rust 1.96.0, and include additional protocol checks and troubleshooting.

The supplementary ZedG Mac trace tested commit `0c863306f829000bef41b7824fa5307b90e6321f`, with extension ID `live-templates`. Its records were added in `518a0d04a55909bb0c5e7d4c2854e20856b1fd91`; neither commit is claimed as the official-Zed manual-test commit.

On 2026-10-06, direct Linux inspection confirmed Fedora 44 (KDE), kernel `7.2.8-200.fc44.x86_64`, and x86_64. The desktop entry identifies the application as ZedG, and its startup log reports `1.22.0+stable.364.76659a55a8c10ed355a070f8764a0b1733e3c115`. The existing development extension index still registers `live-templates`. The configured installed native server also passed the pre-publication LSP end-to-end harness during this inspection. These observations establish the current environment and protocol behavior; they do not identify the exact commit used in the earlier Linux manual checks.

The earlier environment inspections found registrations using the old ID `live-templates`. Both subsequent official-Zed runs confirmed the renamed `live-templates-lsp`; the maintainer reported successful manual testing on each platform.

## Automated checks for this publication preparation

The following checks passed on Linux on 2026-10-06 using Rust 1.96.0 and the updated publication source. The LSP test harness was subsequently removed from the repository at the maintainer's request; its historical results remain below.

| Check | Result |
| --- | --- |
| Native `server` build with the committed dependency lockfile | Passed |
| Pre-publication LSP end-to-end harness against the freshly built server | Passed: `dt`, `todo`, `ctx`, `@dt`; Bash/Python; UTF-16 range; cursor marker; escaping; configuration reload/removal/restoration |
| Release WASM build for `wasm32-wasip2` | Passed with Rust 1.96.0 and official registry CI's Rust 1.90.0 |
| macOS ARM64 native release server and local WASM development build | Passed with Rust 1.96.0 |
| Official registry CLI packaging | Passed with CLI revision `9ee3c503a4bbbc6b4a0f8a789acca4871d773223` and Rust 1.90.0; archive contains only `extension.toml` and `extension.wasm` |
| Registry validation and sorting | Official ID, manifest, version, HTTPS submodule, location and MIT-license validators passed; `pnpm sort-extensions` completed |
| Registry type checking and existing tests | `pnpm build` passed; `pnpm test`: 152 tests passed |
| `cargo fmt --all --check` | Passed |
| Manifest ID, version, unchanged `templates` server ID, and local documentation links | Passed |
| Credential-pattern scan of publication files | No matches found |

Build and format commands from a checkout without the parent workspace's local Cargo source override:

```sh
cargo build -p server --locked
cargo build -p extension --release --target wasm32-wasip2 --locked
cargo fmt --all --check
```

These automated checks are separate from the maintainer's manual official-Zed results recorded above.

## Changes and new-ID follow-up

Compared with the earlier UI-tested baseline `0c86330`, publication preparation changed the extension ID, author and repository metadata, translated the missing-server-path error into English, and updated example descriptions and documentation. The template engine, native LSP server, dependencies, and `templates` language server ID are unchanged. The native build, LSP end-to-end check, WASM build, and format check were repeated successfully after these changes.

On 2026-10-06, an initial isolated official Zed 1.22.0 Linux installation attempt against `84b1660` stopped because the local Rust mirror returned HTTP 404 for the `wasm32-wasip2` standard-library component. A subsequent instance launched with `RUSTUP_TOOLCHAIN=1.96.0` registered `live-templates-lsp` as a development extension from `bd55a66`.

The first completion attempt in that instance failed with the extension's English missing-server-path error. Adding ignored project settings for the native server and template configuration, then restarting the language server, resolved the setup issue. Process inspection confirmed the native server was a child of the official Zed instance. The maintainer then confirmed manual testing passed. The local test settings are not part of the published extension.

The Mac run used a freshly built ARM64 release server and a Linux-built WASM artifact registered in an isolated development-extension directory. Official Zed logs confirmed worktree trust and server startup; the maintainer confirmed manual testing passed. Source installation initially encountered the same Rust-mirror 404. The official component archive was verified against its SHA-256 and installed through rustup's cache. Subsequent Mac-local WASM builds passed, both from the project directory and from `/tmp` with `--manifest-path`. These build checks are separate from the UI run, which used the Linux-built artifact.

## Registry submission source

The extension ID changed from `live-templates` to `live-templates-lsp`; the language server ID `templates` and configuration paths are unchanged. Replace the old development installation before checking the new ID.

The earlier prepared registry entry used version `0.1.0` and pinned submodule commit `bd55a6654976e4aeaa1fd6a38af446588870e696`, the source used for both official-Zed manual runs. It is reachable from the public repository's `main` branch. Subsequent documentation updates record the completed checks without changing runtime source.

The manual checklist provided for these runs was:

- Install the development extension and open a Markdown document with `templates` enabled.
- Expand a configured trigger, confirm the replacement, and check the `$END$` cursor position.
- Run a command variable, then edit the template configuration and confirm the next completion uses it.
- Check a trigger after Chinese text or an emoji and confirm the replacement range and undo behavior.

The maintainer confirmed the functional runs passed. Individual cases beyond those confirmations should not be inferred from logs alone; the protocol cases listed above have separate automated evidence. Zed requires manual testing of the submitted commit: [publishing prerequisites](https://zed.dev/docs/extensions/publishing/prerequisites).
