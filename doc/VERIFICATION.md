# Verification record

## Manual checks reported by the maintainer

On 2026-10-06, the maintainer confirmed that the extension's functionality had been manually verified in Zed on both Linux and macOS.

| Platform | Reported result | Exact test date | Zed version | Tested commit |
| --- | --- | --- | --- | --- |
| Linux | Functional checks passed, confirmed by the maintainer | Not recorded | Not recorded | Not recorded |
| macOS | Functional checks passed, confirmed by the maintainer | Not recorded | Not recorded | Not recorded |
| Windows | Not verified; support is not claimed | — | — | — |

The macOS environment was previously inspected as arm64, macOS 26.6.2, with Zed 1.21.0 and Rust 1.96.0. Those observations describe that environment; they do not establish the version or commit used in the maintainer's later manual checks. A per-case manual checklist was not supplied in this publishing preparation pass.

The repository was clean at baseline commit `0c863306f829000bef41b7824fa5307b90e6321f` before the publication changes. This is a preparation baseline, not a claimed manual-test commit.

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
