# Live Templates LSP design

English | [简体中文](DESIGN.zh-CN.md)

## Problem and functionality

Expand configured Markdown abbreviations into notes, dated entries, and tasks. Runtime variables provide date, time, hostname, and values computed by user commands. The maintainer has confirmed manual functional checks on Linux and macOS. The inspected Linux editor and documented macOS UI tests use ZedG 1.22.0; Windows is unverified. See the [platform test records](TESTING.md) and [verification summary](VERIFICATION.md).

A template can contain:

```text
# $date$ $time$ $host$
$END$
---
```

After the user confirms the completion, Zed replaces the abbreviation and places the cursor on the second line.

- Users define triggers and template bodies in TOML. The longest matching trigger wins.
- Built-in variables are `date`, `time`, and `host`. User commands can add variables or override built-ins.
- `$END$` marks the final cursor position; omission places it at the end.
- Template configuration reloads on the next completion. Scripts are read on each execution.

The extension ID is `live-templates-lsp`; the language server ID remains `templates`. Installation, configuration, and the command interface are documented in the [README](../README.md).

## Workflow

1. The user writes `templates.toml`, including triggers, template bodies, and optional variable commands.
2. The Zed WASM extension reads the configured server path and starts the native LSP server. Zed supplies document content and completion positions.
3. The server reloads changed configuration and selects the longest complete trigger matching the document language and cursor position.
4. The server creates one time/document snapshot as JSON context. Built-ins use that snapshot; commands receive it through stdin and environment variables, and return values through stdout. Repeated variables are computed once per render.
5. The renderer inserts variable values as literal text, escapes snippet characters, and converts `$END$` to `$0`. The server returns the snippet and replacement range, discarding results for documents whose version has changed.
6. Zed applies the completion and provides cursor navigation and undo.

Unchanged configuration reuses the parsed value. Invalid configuration preserves the previous valid version; deletion clears templates, and restoration reloads them.

Command failure or timeout cancels the completion. Commands and timestamps are evaluated when generating the candidate, so a completion refresh can rerun commands, and confirmation does not refresh the timestamp.

## Code structure

One Rust workspace has three packages and builds two artifacts:

| Package / location | Responsibility | Artifact |
| --- | --- | --- |
| `extension` / `src/lib.rs` | Zed integration, settings, native server startup | `extension.wasm` |
| `server` / `crates/server/src/` | LSP requests, document state, reload, trigger matching, completions | Native `server` executable |
| `core` / `crates/core/src/` | Configuration and template parsing, context, commands, snippet rendering | Library linked into `server` |

Zed and the native server exchange LSP messages over stdin/stdout. The server runs user commands separately and writes diagnostics to stderr. The native server is built and configured separately from the WASM extension.

- `server/main.rs` connects configuration, matching, rendering, and completion responses.
- `server/documents.rs` tracks documents and converts UTF-16 positions and trigger boundaries.
- `core/config.rs` and `core/parser.rs` parse configuration and template nodes.
- `core/context.rs` creates the time, hostname, project, and document snapshot.
- `core/command.rs` and `core/renderer.rs` run variable commands and assemble snippets.

`examples/` contains sample configuration and scripts. `scripts/smoke.py` is the single LSP end-to-end check.
