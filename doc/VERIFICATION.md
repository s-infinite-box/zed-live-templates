# Verification record

## Manual checks reported by the maintainer

On 2026-10-06, the maintainer explicitly confirmed successful manual functional checks in the official Zed distribution on Linux x86_64 and macOS ARM64. These official-Zed results are the primary compatibility evidence. Earlier ZedG logs and environment inspections are supplementary evidence.

| Platform | Reported result | Confirmation date | Editor | Tested commit |
| --- | --- | --- | --- | --- |
| Linux x86_64 | Functional checks passed, confirmed by the maintainer | 2026-10-06 | Official Zed | Not separately recorded |
| macOS ARM64 | Functional checks passed, confirmed by the maintainer | 2026-10-06 | Official Zed | Not separately recorded |
| Windows | Not verified; support is not claimed | — | — | — |

The official-Zed confirmation and earlier per-case results are documented in the [platform test records](TESTING.md), also available in [Chinese](TESTING.zh-CN.md). The official macOS installation was observed reporting Zed 1.22.0; exact versions and dates of the official manual runs were not separately recorded. Supplementary Mac records identify macOS 26.6.2, ARM64, Rust 1.96.0, and ZedG 1.22.0, including the native build, LSP check, and setup troubleshooting.

The supplementary ZedG Mac trace tested commit `0c863306f829000bef41b7824fa5307b90e6321f`, with extension ID `live-templates`. Its records were added in `518a0d04a55909bb0c5e7d4c2854e20856b1fd91`; neither commit is claimed as the official-Zed manual-test commit.

On 2026-10-06, direct Linux inspection confirmed Fedora 44 (KDE), kernel `7.2.8-200.fc44.x86_64`, and x86_64. The desktop entry identifies the application as ZedG, and its startup log reports `1.22.0+stable.364.76659a55a8c10ed355a070f8764a0b1733e3c115`. The existing development extension index still registers `live-templates`. The configured installed native server also passed the pre-publication LSP end-to-end harness during this inspection. These observations establish the current environment and protocol behavior; they do not identify the exact commit used in the earlier Linux manual checks.

Both inspected development registrations still use the old ID `live-templates`. The maintainer's official-Zed confirmation does not specify whether the renamed `live-templates-lsp` was tested at the final submission commit; the final submission check below remains pending until that exact commit is confirmed.

## Automated checks for this publication preparation

The following checks passed on Linux on 2026-10-06 using Rust 1.96.0 and the updated publication source. The LSP test harness was subsequently removed from the repository at the maintainer's request; its historical results remain below.

| Check | Result |
| --- | --- |
| Native `server` build with the committed dependency lockfile | Passed |
| Pre-publication LSP end-to-end harness against the freshly built server | Passed: `dt`, `todo`, `ctx`, `@dt`; Bash/Python; UTF-16 range; cursor marker; escaping; configuration reload/removal/restoration |
| Release WASM build for `wasm32-wasip2` | Passed |
| `cargo fmt --all --check` | Passed |
| Manifest ID, version, unchanged `templates` server ID, and local documentation links | Passed |
| Credential-pattern scan of publication files | No matches found |

Build and format commands from a checkout without the parent workspace's local Cargo source override:

```sh
cargo build -p server --locked
cargo build -p extension --release --target wasm32-wasip2 --locked
cargo fmt --all --check
```

These checks exercise the server and build the extension; they do not establish a new manual Zed test result for the changed extension ID.

## Changes and new-ID follow-up

Compared with the earlier UI-tested baseline `0c86330`, publication preparation changed the extension ID, author and repository metadata, translated the missing-server-path error into English, and updated example descriptions and documentation. The template engine, native LSP server, dependencies, and `templates` language server ID are unchanged. The native build, LSP end-to-end check, WASM build, and format check were repeated successfully after these changes.

On 2026-10-06, an isolated official Zed 1.22.0 Linux instance was launched and the development-extension installation action was attempted against source commit `84b1660`. Installation stopped because the local Rust mirror returned HTTP 404 for the `wasm32-wasip2` standard-library component. This attempt is not recorded as a successful install or UI regression. The maintainer will complete installation and manual checks of the renamed ID.

## Final submission check

The extension ID changed from `live-templates` to `live-templates-lsp`; the language server ID `templates` and configuration paths are unchanged. Replace the old development installation before checking the new ID.

Before submitting a registry PR, the maintainer should manually check the exact extension submodule commit that will be submitted and fill in the platform, architecture, Zed version, date, and commit:

- Install the development extension and open a Markdown document with `templates` enabled.
- Expand a configured trigger, confirm the replacement, and check the `$END$` cursor position.
- Run a command variable, then edit the template configuration and confirm the next completion uses it.
- Check a trigger after Chinese text or an emoji and confirm the replacement range and undo behavior.

Record only checks actually performed; these final checks are pending until confirmed. Zed requires manual testing of the submitted commit: [publishing prerequisites](https://zed.dev/docs/extensions/publishing/prerequisites).
