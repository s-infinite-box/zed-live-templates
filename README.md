# Live Templates LSP for Zed

English | [简体中文](README.zh-CN.md)

Expand user-defined abbreviations into Markdown templates with dynamic variables. Select a completion to replace the abbreviation and move the cursor to `$END$`. Triggers, template bodies, and command variables belong to the user's configuration; `dt` and `todo` are examples.

The extension ID is `live-templates-lsp`. The language server name used in Zed settings is `templates`.

The maintainer has confirmed manual functional checks in official Zed on Linux x86_64 and macOS ARM64. Windows has not been verified and is not claimed as supported. See the [platform test records](doc/TESTING.md) and [verification summary](doc/VERIFICATION.md) for the official-Zed results, supplementary environment checks, and final submission checks.

## Build and configure

The project uses Rust 1.96.0 and the `wasm32-wasip2` target. From the repository root:

```sh
cargo build -p server --release
cargo build -p extension --release --target wasm32-wasip2
```

The native server is `target/release/server`; the WASM artifact is `target/wasm32-wasip2/release/extension.wasm`. Build the native server on the machine and architecture where it will run. Bash, Python, or other interpreters used by your command variables must also be available there. The macOS UI checks used a Linux-built WASM artifact and a Mac-built native server. Subsequent Mac-local WASM development builds also passed; see the test records for their scope.

For a development installation, run `zed: install dev extension` in Zed's command palette and select this repository. After registry publication, you can install `live-templates-lsp` from Zed's extensions view. In either case, build the native server separately and merge the following into your Zed settings, replacing both paths with absolute paths:

```json
{
  "languages": {
    "Markdown": {
      "language_servers": ["templates", "..."]
    }
  },
  "lsp": {
    "templates": {
      "binary": {
        "path": "/path/to/zed-live-templates/target/release/server"
      },
      "initialization_options": {
        "config_path": "/path/to/zed-live-templates/examples/templates.toml"
      }
    }
  }
}
```

`"..."` keeps the other language servers enabled. Restart the Markdown language servers, type a configured trigger such as `dt`, select the template completion, and confirm it using your keymap. If the menu does not appear automatically, run `editor: show completions`.

If you installed the previous development extension with ID `live-templates`, uninstall it before installing `live-templates-lsp`. Keep your existing `lsp.templates` settings and template configuration paths.

By default the server reads `$XDG_CONFIG_HOME/zed-live-templates/templates.toml`, or `~/.config/zed-live-templates/templates.toml` when `XDG_CONFIG_HOME` is unset. You can also pass `server --config /absolute/path/templates.toml`; the command-line path takes precedence over `initialization_options.config_path`. Without a configuration file, the server provides no completions.

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

Dates and times are captured when the completion is generated. Confirming a candidate later can insert an earlier timestamp. Automatic menu display can depend on Zed's completion behavior and other language servers. Windows configuration lookup and command process cleanup have not been verified.

## Design and scope

See the [design document](doc/DESIGN.md) for functionality, workflow, and code structure.

This project supplies user-defined templates with runtime values and command context through LSP snippet completions. The existing [live-template extension](https://github.com/jekst/zed-snippets) provides a collection of Go snippets; this project's configurable runtime variables are the intended distinction. Acceptance of this use of the extension API is subject to Zed maintainers' review.

API references: [developing extensions](https://zed.dev/docs/extensions/developing-extensions), [language servers](https://zed.dev/docs/extensions/languages#language-servers), and [choosing language servers](https://zed.dev/docs/configuring-languages#choosing-language-servers).

## License

[MIT](LICENSE).
