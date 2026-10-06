# Platform test records

English | [简体中文](TESTING.zh-CN.md)

The primary compatibility result is the maintainer's confirmed manual testing in official Zed on Linux and macOS. The renamed extension has passed new manual checks in official Zed 1.22.0 on both platforms, using source `bd55a66`. The earlier ZedG inspection and logs below are supplementary test evidence; see the [verification summary](VERIFICATION.md).

## Official Zed manual verification

On 2026-10-06, the maintainer explicitly confirmed that functionality had also been tested in the official Zed distribution on both platforms.

| Platform | Editor | Reported result |
| --- | --- | --- |
| Linux x86_64 | Official Zed | Functional checks passed, confirmed by the maintainer |
| macOS ARM64 | Official Zed | Functional checks passed, confirmed by the maintainer |

The official macOS installation was observed reporting Zed 1.22.0. The earlier official runs' exact dates and extension commits were not separately supplied. The precise versions and commits in the supplementary records below refer to those recorded runs and should not be reassigned to the official-Zed runs.

### Renamed extension: official Linux Zed

On 2026-10-06, the maintainer confirmed manual testing passed with extension source `bd55a66`, ID `live-templates-lsp`, and official Zed 1.22.0 on Fedora 44 x86_64. The application used an isolated data directory and `RUSTUP_TOOLCHAIN=1.96.0`; its extension index confirmed a development registration under the new ID.

The first `dt` attempt failed because this instance had no `lsp.templates.binary.path`. Ignored project settings supplied absolute paths to the freshly built native server and `examples/templates.toml`, with `templates` enabled for Markdown. After restarting the language server, process inspection confirmed server startup under this official Zed instance, and the maintainer reported no problems in manual testing. This fixes local test setup; no runtime source was changed.

### Renamed extension: official macOS Zed

On 2026-10-06, source `bd55a66` was updated on macOS 26.6.2 ARM64 and its native release server was rebuilt with Rust 1.96.0. Official Zed 1.22.0 loaded a Linux-built WASM artifact under development ID `live-templates-lsp` in an isolated directory. Project settings selected the Mac-native server and `examples/templates.toml`. Logs confirmed worktree trust and server startup, and process inspection confirmed the server was a child of that official Zed instance. The maintainer subsequently confirmed manual testing passed.

The source-installation action initially failed because the configured Rust mirror returned HTTP 404 for the WASM standard library. The official component archive was downloaded, verified against SHA-256 `17a511eade6b74a86a31af9f7498d416a717472a91d11b180ac760944e69599e`, and installed through rustup. Mac-local WASM development builds then passed, including a build from the repository directory. A separate build from `/tmp` using `--manifest-path` avoided the parent workspace's Cargo mirror while downloading dependencies. The UI run used the staged Linux-built artifact; a successful repeat of Zed's source-installation UI action was not separately recorded.

## Supplementary Linux environment and installed server

Inspected directly on 2026-10-06:

| Item | Observed value |
| --- | --- |
| OS | Fedora Linux 44, KDE Plasma Desktop Edition |
| Kernel / architecture | `7.2.8-200.fc44.x86_64`, x86_64 |
| Editor | Desktop entry: ZedG; startup version: `1.22.0+stable.364.76659a55a8c10ed355a070f8764a0b1733e3c115` |
| Rust / Cargo used for publication checks | 1.96.0 |
| Python | 3.14.7 |
| Existing development registration | `live-templates`, pointing to this repository |
| LSP server ID | `templates` |
| Configured installed native server | ELF 64-bit executable, x86_64, in the old extension's work directory |
| Repository at inspection | `518a0d04a55909bb0c5e7d4c2854e20856b1fd91` |

The CLI's version output omits the display version but includes SHA `76659a55a8c10ed355a070f8764a0b1733e3c115`. The application's startup log identifies the full version above, and its desktop entry identifies the application as ZedG.

The maintainer previously confirmed manual functionality on Linux. The exact date and extension commit of those manual checks were not recorded. The repository commit in the table is the current checkout, not a claimed manual-test commit.

The existing Zed-configured native server was checked directly with the pre-publication LSP end-to-end harness. That test script was subsequently removed from the repository at the maintainer's request.

Result: passed `dt`, `todo`, `ctx`, `@dt`, Bash/Python variables, Chinese/emoji UTF-16 ranges, cursor markers, snippet escaping, and configuration reload/removal/restoration. The server executable was not replaced during this inspection. This check verifies the installed server's protocol behavior and does not constitute a new UI check.

## Supplementary macOS ZedG test trace

On 2026-10-06, the native server build, LSP end-to-end check, and ZedG UI checks passed on Apple Silicon. The maintainer performed the UI actions and confirmed the results; builds, protocol checks, and server startup were checked through command-line evidence.

| Item | Tested environment |
| --- | --- |
| OS / architecture | macOS 26.6.2, Darwin 25.6.0, ARM64 |
| Rust / Cargo | 1.96.0 |
| Python | 3.14.4, existing virtual environment |
| Editor | ZedG 1.22.0 |
| UI-tested code | `0c863306f829000bef41b7824fa5307b90e6321f` |
| UI-tested extension ID | `live-templates` |
| Configuration | `examples/templates.toml`; the smoke check adds templates to a temporary copy |

The records were committed in `518a0d0`. A direct SSH inspection confirmed the clean Mac checkout contains that commit, ZedG's CLI reports 1.22.0, and the isolated test data directory still registers `live-templates`. The native recheck and smoke logs show successful completion. Those observations do not establish a new UI check of `live-templates-lsp`.

### Automated checks

The native server was built with the existing Rust `stable` toolchain, version 1.96.0. Commands were run outside the parent project's directory, using `--manifest-path` to avoid that parent's Cargo mirror override:

```sh
project_dir="/absolute/path/to/zed-live-templates"
cargo +stable build --manifest-path "$project_dir/Cargo.toml" \
  -p server --release --locked --offline
file "$project_dir/target/release/server"
```

The build produced a `Mach-O 64-bit executable arm64`. Offline mode requires cached dependencies; remove `--offline` for a first build if needed.

The existing Python environment ran the pre-publication LSP end-to-end harness against `target/release/server`. That script was subsequently removed from the repository; the following table records the completed historical checks.

| Case | Result |
| --- | --- |
| `dt` date template and `todo` task template | Passed |
| Bash weekday/hostname scripts | Passed |
| Python JSON context and escaping of `$` and `}` | Passed |
| UTF-16 range in `记录🙂 dt` | Passed; only the trigger was replaced |
| `$END$` rendered as snippet `$0` | Passed |
| Longest match for `dt` and `@dt` | Passed |
| Configuration change, deletion, and restoration | Passed |
| LSP shutdown and process exit | Passed |

Final output:

```text
OK: dt / todo / ctx / @dt, Bash / Python, UTF-16 range, cursor, snippet escaping, configuration reload / removal / restoration
```

### Manual UI checks

The development extension was loaded in an isolated data directory. Project-level Zed settings selected `templates`, the Mac-native server, and the template configuration using absolute paths. The document was Markdown.

| Maintainer action | Observed result |
| --- | --- |
| Type `dt` character by character | Template candidate appeared automatically |
| Select the `dt` candidate and press Enter | Replaced the abbreviation with weekday, date, time, and hostname |
| Type at the resulting cursor position | Cursor was on the blank line between the heading and `---` |
| Type `todo` and confirm | Expanded to `- [ ] `, with cursor at the end |
| Expand `记录🙂 dt` | Preserved `记录🙂 ` and replaced only `dt` |
| Type and expand `dt` on another line | Candidate appeared and expanded again |

The maintainer confirmed all cases passed.

### Startup troubleshooting

The first `dt` attempt produced no candidate. Inspection found two setup issues:

1. The normal macOS CLI launch did not pass the isolated `--user-data-dir` through to the application process. The file opened in the main instance, which had its existing extensions but not this test extension. Launching the application executable directly selected the intended isolated data directory.
2. The new instance waited for worktree trust before starting `templates`. After the maintainer trusted the project, logs confirmed server startup and the candidates appeared.

The isolated instance was launched with:

```sh
project_dir="/absolute/path/to/zed-live-templates"
data_dir="$project_dir/.tmp/zed-data"
/Applications/ZedG.app/Contents/MacOS/zed \
  --user-data-dir "$data_dir" "$project_dir"
```

The development extension had already been registered in that data directory. In a fresh directory, install the extension and configure the language server in that instance. For official Zed, the corresponding application path is `/Applications/Zed.app/Contents/MacOS/zed`. `--new` creates a workspace; it does not establish a separate application process or data directory.

Project-level configuration requires a trusted worktree. Use the title-bar trust indicator or `workspace: toggle worktree security` when appropriate. If no candidate appears, inspect the active extension and server first, then try `editor: show completions`. Open logs with `zed: open log`.

References from the Mac test record: [Zed CLI source](https://github.com/zed-industries/zed/blob/76659a55a8c10ed355a070f8764a0b1733e3c115/crates/cli/src/main.rs), [worktree trust](https://zed.dev/docs/worktree-trust), and [development extensions](https://zed.dev/docs/extensions/developing-extensions).

## Scope

- The earlier ZedG run used a Linux-built `extension.wasm` and a Mac-built native server. Mac-local WASM builds and the current official-Zed run are recorded separately above; a successful repeat of the source-installation UI action was not separately recorded.
- The detailed Mac trace above was collected with ZedG 1.22.0. The earlier official-Zed startup issue was a setup failure; the maintainer subsequently confirmed successful official-Zed testing, recorded above.
- macOS Intel, other editor versions, and languages beyond Markdown were not checked.
- The earlier registrations used `live-templates`. The new `live-templates-lsp` passed official Linux and macOS manual testing. The prepared registry entry pins the tested source `bd55a66`.
- Dates and times come from candidate generation; confirming a candidate after the menu spans a minute boundary can insert the earlier time.
