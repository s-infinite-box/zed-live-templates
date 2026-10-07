# Live Templates LSP for Zed

English | [简体中文](README.zh-CN.md)

Expand user-defined abbreviations into Markdown templates with date, time, hostname, and optional command variables. Confirm a completion to replace the abbreviation and move the cursor to `$END$`. Triggers and template bodies belong to your configuration; `dt` and `todo` are examples.

The extension ID is `live-templates-lsp`; the language server ID in Zed settings is `templates`. The extension currently registers **Markdown only**.

## Install and configure

Install **Live Templates** from Zed's extensions view once the registry submission is accepted. Until then, see [development installation](#development-installation).

The extension automatically downloads the matching precompiled server from this repository's [GitHub releases](https://github.com/s-infinite-box/zed-live-templates/releases). Zed starts it and caches it in the extension's work directory. **Normal users do not need Rust or a server executable path.** The first start needs access to GitHub release assets; subsequent starts reuse the cached version, including offline. An extension upgrade selects its matching server version; a failed download is retried on the next language-server start.

Precompiled servers are published for all six combinations:

| OS | x86_64 | ARM64 |
| --- | --- | --- |
| Linux | static musl | static musl |
| macOS | Intel | Apple Silicon |
| Windows | MSVC | MSVC |

Create `templates.toml` using the example below. The default locations are:

| Platform | Default template configuration |
| --- | --- |
| Linux / macOS | `~/.config/zed-live-templates/templates.toml` |
| Windows | `%APPDATA%\zed-live-templates\templates.toml` |
| Any platform with `XDG_CONFIG_HOME` set | `$XDG_CONFIG_HOME/zed-live-templates/templates.toml` |

If your Zed settings already restrict Markdown language servers, add `"templates"` to that list. To use a different template file, merge the following into your Zed settings, replacing the template path with an absolute path (Windows JSON paths need escaped backslashes or forward slashes):

```json
{
  "languages": {
    "Markdown": {
      "language_servers": ["templates", "..."]
    }
  },
  "lsp": {
    "templates": {
      "initialization_options": {
        "config_path": "/absolute/path/templates.toml"
      }
    }
  }
}
```

`"..."` keeps other language servers enabled. Open a Markdown file, type a configured trigger, select the template completion, and confirm it using your keymap. If the menu does not appear automatically, run `editor: show completions`. Project settings require a trusted worktree. Without a template configuration, the server provides no completions.

### Optional server override

Advanced users can set `lsp.templates.binary.path` to an absolute native server path. This bypasses downloading entirely. `binary.arguments` and `binary.env` remain available with both the automatic server and a custom path. A `--config` argument takes precedence over `initialization_options.config_path`.

```json
{
  "lsp": {
    "templates": {
      "binary": {
        "path": "/absolute/path/custom/server",
        "arguments": ["--config", "/absolute/path/templates.toml"]
      }
    }
  }
}
```

If you installed the old development ID `live-templates`, uninstall it before installing `live-templates-lsp`. Keep your template configuration and `lsp.templates` settings; remove `binary.path` to switch to automatic installation.

## Development installation

Rust is required only to build from source. Builds and the release workflow are verified with Rust 1.90.0; development builds also passed with 1.96.0. Install the `wasm32-wasip2` target and run `zed: install dev extension` from Zed's command palette, selecting this repository. The development extension downloads the released server by default. To test native source changes, build locally and use the optional override:

```sh
cargo build -p server --release --locked
cargo build -p extension --release --target wasm32-wasip2 --locked
cargo test --workspace --locked
```

The native output is `target/release/server` (`server.exe` on Windows); the WASM output is `target/wasm32-wasip2/release/extension.wasm`. The registry package contains the manifest and WASM extension only. Native servers are distributed through GitHub releases by [.github/workflows/server.yml](.github/workflows/server.yml).

## Define templates

```toml
version = 1

[formats]
date = "%Y/%-m/%-d"
time = "%H:%M"
timezone = "local"

[[templates]]
id = "daily_note"
trigger = "dt"
description = "Insert a dated note"
languages = ["Markdown"]
body = '''# $date$ $time$ $host$
$END$
---'''

[[templates]]
id = "task"
trigger = "todo"
description = "Insert a task"
languages = ["Markdown"]
body = '- [ ] $END$'
```

Built-in variables are `$date$`, `$time$`, and `$host$`. `$END$` marks the final cursor position; if omitted, the cursor goes to the end. `timezone` accepts `local` or `UTC`.

Use `\$` to insert a literal dollar sign. Incomplete variable markers such as `$PATH` remain unchanged. Command output is inserted as literal text and is not parsed as another template; snippet special characters are escaped.

You can add command variables or override built-in variables. Template-level variables, such as `[templates.variables.host]`, take precedence over global `[variables.host]`, which takes precedence over the built-in value. `END` is reserved for the cursor.

When several triggers match, the longest one wins. For example, `@dt` takes precedence over `dt`, regardless of configuration order.

## Commands and context

The first element of a `command` array is the executable; the remaining elements are passed as arguments. Choose any interpreter explicitly:

```toml
[variables.host]
command = ["bash", "variables/short_host.sh"]
timeout_ms = 1000

[variables.symbol_name]
command = ["python3", "variables/symbol_name.py"]
```

The default working directory is the configuration file's directory. Optional `cwd` accepts a path relative to that directory or an absolute path. Executable names are looked up through `PATH`; relative executable paths with directory components are resolved against the configuration directory.

The server sends a JSON context to stdin and closes it after writing. Python scripts can use `json.load(sys.stdin)` and Node scripts can read file descriptor 0. Common fields are also supplied as environment variables:

- `TPL_WEEKDAY`: Monday is `1`, Sunday is `7`.
- `TPL_DATE`, `TPL_TIME`, `TPL_HOST`: the original built-in values.
- `TPL_CTX_VERSION`, `TPL_VARIABLE`, `TPL_TEMPLATE_ID`, `TPL_TRIGGER`.
- `TPL_PROJECT_ROOT`, `TPL_PROJECT_NAME`, when available.
- `TPL_DOCUMENT_URI`, `TPL_DOCUMENT_PATH`, `TPL_DOCUMENT_NAME`, `TPL_LANGUAGE`, when available.

See [examples/ctx.json](examples/ctx.json) for the full structure. All variables share one time and document snapshot; a variable used repeatedly is evaluated once per template render. `ctx.now.timezone` contains the UTC offset and `ctx.document.language` contains the LSP language ID, such as `markdown`. `ctx.symbol` and `ctx.selection` are currently `null`.

Stdout becomes the variable value, including spaces and trailing newlines; stderr is diagnostic output. A nonzero exit or timeout cancels the template completion. The default timeout is 1000 ms and stdout is limited to 64 KiB. Use `printf` or `sys.stdout.write` to avoid an unwanted trailing newline.

Commands run when generating a completion and can run again when completions refresh. Use them to compute values, and only enable configurations and scripts you trust. Configuration changes are loaded on the next completion; invalid configuration preserves the last valid version. Removing the configuration file clears active templates on the next completion, and restoring it reloads them. Scripts are read on each execution.

[examples/templates.toml](examples/templates.toml) also demonstrates a user script that formats weekday numbers in Chinese. This localized output is template content, not a required format.

## Verification and limitations

The maintainer has confirmed manual functional checks in official Zed on Linux and macOS. The pre-publication LSP checks also passed; their harness was subsequently removed from this repository. Results and the manual checklist for the renamed extension are recorded in [doc/VERIFICATION.md](doc/VERIFICATION.md).

Dates and times are captured when the completion is generated. Confirming a candidate later can insert an earlier timestamp. Automatic menu display can depend on Zed's completion behavior and other language servers. Custom command interpreters must be available locally; built-in variables require none. On Windows, timeouts terminate the immediate command process; descendant-process cleanup is currently guaranteed only on Unix.

## Design and scope

See the [design document](doc/DESIGN.md) for functionality, workflow, and code structure.

This project supplies user-defined templates with runtime values and command context through LSP snippet completions. The existing [live-template extension](https://github.com/jekst/zed-snippets) provides a collection of Go snippets; this project's configurable runtime variables are the intended distinction. Acceptance of this use of the extension API is subject to Zed maintainers' review.

API references: [developing extensions](https://zed.dev/docs/extensions/developing-extensions), [language servers](https://zed.dev/docs/extensions/languages#language-servers), and [choosing language servers](https://zed.dev/docs/configuring-languages#choosing-language-servers).

## License

[MIT](LICENSE).
