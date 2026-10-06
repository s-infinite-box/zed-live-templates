# Verification record

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

The prepared registry entry uses version `0.1.0` and pins submodule commit `bd55a6654976e4aeaa1fd6a38af446588870e696`, the source used for both official-Zed manual runs. It is reachable from the public repository's `main` branch. Subsequent documentation updates record the completed checks without changing runtime source.

The manual checklist provided for these runs was:

- Install the development extension and open a Markdown document with `templates` enabled.
- Expand a configured trigger, confirm the replacement, and check the `$END$` cursor position.
- Run a command variable, then edit the template configuration and confirm the next completion uses it.
- Check a trigger after Chinese text or an emoji and confirm the replacement range and undo behavior.

The maintainer confirmed the functional runs passed. Individual cases beyond those confirmations should not be inferred from logs alone; the protocol cases listed above have separate automated evidence. Zed requires manual testing of the submitted commit: [publishing prerequisites](https://zed.dev/docs/extensions/publishing/prerequisites).
