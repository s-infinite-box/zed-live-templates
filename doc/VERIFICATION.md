# Verification record

## Manual checks reported by the maintainer

On 2026-10-06, the maintainer confirmed manual functional checks on Linux and macOS. Current environment inspection and the documented macOS checks identify the editor as ZedG, a Zed build with a Simplified Chinese interface.

| Platform | Reported result | Exact test date | Zed version | Tested commit |
| --- | --- | --- | --- | --- |
| Linux x86_64 | Functional checks passed, confirmed by the maintainer | Not recorded | Current installation observed: ZedG 1.22.0+stable.364 | Not recorded |
| macOS ARM64 | Functional checks passed, confirmed by the maintainer | 2026-10-06 | ZedG v1.22.0 | `0c863306f829000bef41b7824fa5307b90e6321f` |
| Windows | Not verified; support is not claimed | — | — | — |

The macOS per-case results are documented in the [platform test records](TESTING.md), also available in [Chinese](TESTING.zh-CN.md): macOS 26.6.2, ARM64, Rust 1.96.0, and ZedG 1.22.0. The maintainer confirmed all listed UI cases passed. The record includes the native build, LSP check, extension-directory mismatch, and worktree-trust setup.

The macOS-tested commit was `0c863306f829000bef41b7824fa5307b90e6321f`, with extension ID `live-templates`. Its records were added in `518a0d04a55909bb0c5e7d4c2854e20856b1fd91`; the record's commit is not the UI-tested commit.

On 2026-10-06, direct Linux inspection confirmed Fedora 44 (KDE), kernel `7.2.8-200.fc44.x86_64`, and x86_64. The desktop entry identifies the application as ZedG, and its startup log reports `1.22.0+stable.364.76659a55a8c10ed355a070f8764a0b1733e3c115`. The existing development extension index still registers `live-templates`. The configured installed native server also passed `scripts/smoke.py` during this inspection. These observations establish the current environment and protocol behavior; they do not identify the exact commit used in the earlier Linux manual checks.

Both inspected development registrations still use the old ID `live-templates`. The renamed `live-templates-lsp` has not yet been confirmed by a new UI test; the final submission check below remains pending. Complete UI verification in the official Zed distribution has not been established by these ZedG results.

## Automated checks for this publication preparation

The following checks passed on Linux on 2026-10-06 using Rust 1.96.0 and the updated publication source:

| Check | Result |
| --- | --- |
| Native `server` build with the committed dependency lockfile | Passed |
| `scripts/smoke.py` against the freshly built server | Passed: `dt`, `todo`, `ctx`, `@dt`; Bash/Python; UTF-16 range; cursor marker; escaping; configuration reload/removal/restoration |
| Release WASM build for `wasm32-wasip2` | Passed |
| `cargo fmt --all --check` | Passed |
| Manifest ID, version, unchanged `templates` server ID, and local documentation links | Passed |
| Credential-pattern scan of publication files | No matches found |

Equivalent commands from a checkout without the parent workspace's local Cargo source override:

```sh
cargo build -p server --locked
python3 scripts/smoke.py target/debug/server
cargo build -p extension --release --target wasm32-wasip2 --locked
cargo fmt --all --check
```

These checks exercise the server and build the extension; they do not establish a new manual Zed test result for the changed extension ID.

## Final submission check

The extension ID changed from `live-templates` to `live-templates-lsp`; the language server ID `templates` and configuration paths are unchanged. Replace the old development installation before checking the new ID.

Before submitting a registry PR, the maintainer should manually check the exact extension submodule commit that will be submitted and fill in the platform, architecture, Zed version, date, and commit:

- Install the development extension and open a Markdown document with `templates` enabled.
- Expand a configured trigger, confirm the replacement, and check the `$END$` cursor position.
- Run a command variable, then edit the template configuration and confirm the next completion uses it.
- Check a trigger after Chinese text or an emoji and confirm the replacement range and undo behavior.

Record only checks actually performed; these final checks are pending until confirmed. Zed requires manual testing of the submitted commit: [publishing prerequisites](https://zed.dev/docs/extensions/publishing/prerequisites).
