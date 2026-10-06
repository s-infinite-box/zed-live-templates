# Zed 动态模板

[English](README.md) | 简体中文

用户在 Markdown 中输入自己配置的缩写，选择补全候选后确认，插入模板并定位到 `$END$`。模板、自定义变量及其命令都由用户配置，`dt` 和 `todo` 只是示例。

代码分为 Zed WASM 扩展 `extension`、模板库 `core` 和原生 LSP 服务 `server`。当前支持 Markdown；用户已确认在 Linux 和 macOS 的 Zed 中完成手动功能验证。Windows 尚未验证，当前不声明支持。验证来源和待补项见 [验证记录](doc/VERIFICATION.md)。

功能、工作流程和代码架构见 [设计文档](doc/DESIGN.zh-CN.md)。扩展 ID 为 `live-templates-lsp`；Zed 设置中的语言服务器名称仍为 `templates`。

## 构建和接入 Zed

在项目目录运行：

```sh
cargo build -p server --release
cargo build -p extension --release --target wasm32-wasip2
```

项目使用 Rust 1.96.0 和 `wasm32-wasip2` 目标。原生服务位于 `target/release/server`，扩展产物位于 `target/wasm32-wasip2/release/extension.wasm`。

在 macOS 上需重新构建原生 `server`，Linux 的二进制不能直接使用。用户命令所需的 Bash、Python 等解释器也需要在本机可用。

已安装旧 ID `live-templates` 的开发扩展时，先卸载旧扩展，再用本项目目录重新安装 `live-templates-lsp`。现有 `lsp.templates` 设置和模板配置路径继续使用。

在 Zed 命令面板执行 `zed: install dev extension`，选择本项目根目录。Zed 会构建并加载扩展。随后把下面配置合并到 Zed 用户设置中，将两处路径换成实际绝对路径：

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

`"..."` 保留其他语言服务器。重启 Markdown 的语言服务器后，输入示例中的 `dt` 或 `todo`；如果自动补全未弹出，执行 `editor: show completions`。选中模板候选后按当前键位确认；默认通常是 Enter。

默认模板配置路径为 `$XDG_CONFIG_HOME/zed-live-templates/templates.toml`，未设置 `XDG_CONFIG_HOME` 时使用 `~/.config/zed-live-templates/templates.toml`。也可以通过 `server --config /absolute/path/templates.toml` 指定，命令行优先于初始化选项。没有模板配置时不提供候选。

## 配置模板

```toml
version = 1

[formats]
date = "%Y/%-m/%-d"
time = "%H:%M"
timezone = "local"

[variables.week]
command = ["bash", "variables/week_zh.sh"]

[[templates]]
id = "daily_note"
trigger = "dt"
description = "插入日期记录"
languages = ["Markdown"]
body = '''# 周$week$ $date$ $time$ $host$
$END$
---'''

[[templates]]
id = "task"
trigger = "todo"
languages = ["Markdown"]
body = '- [ ] $END$'
```

内置变量只有 `$date$`、`$time$`、`$host$`。`$END$` 表示最终光标位置，省略时光标在末尾。`$week$` 是用户脚本变量，插件提供原始星期编号供脚本转换。`timezone` 支持 `local` 和 `UTC`。

使用 `\$` 输出字面量 `$`；普通 `$PATH` 等不完整变量标记保留原样。变量输出不会再次解析为模板，内部的 `$`、反斜杠和 `}` 会正确转义。

可以增加任意命令变量，也可以覆盖内置变量。模板级 `[templates.variables.host]` 优先于全局 `[variables.host]`，全局定义优先于内置值。`END` 保留给光标。

多个缩写同时匹配时，优先使用最长的缩写。例如同时定义 `dt` 和 `@dt`，输入 `@dt` 会使用后者，与配置顺序无关。

## 命令和上下文

`command` 数组的第一项是程序，后面是参数。插件按原样传递参数，用户显式选择解释器，例如：

```toml
[variables.host]
command = ["bash", "variables/short_host.sh"]
timeout_ms = 1000

[variables.symbol_name]
command = ["python3", "variables/symbol_name.py"]
```

默认工作目录是配置文件所在目录。可选 `cwd` 相对配置目录解析，也接受绝对路径。程序名通过 `PATH` 查找；带路径的程序项相对配置目录解析。脚本和解释器由用户维护。

执行器把完整 `ctx` 以 JSON 写入命令 stdin，写完关闭输入。Python 可以使用 `json.load(sys.stdin)`，Node 可以读取 fd 0。常用字段同时以环境变量传入：

- `TPL_WEEKDAY`：周一为 `1`，周日为 `7`。
- `TPL_DATE`、`TPL_TIME`、`TPL_HOST`：尚未被自定义命令覆盖的内置值。
- `TPL_CTX_VERSION`、`TPL_VARIABLE`、`TPL_TEMPLATE_ID`、`TPL_TRIGGER`。
- `TPL_PROJECT_ROOT`、`TPL_PROJECT_NAME`：有值时提供。
- `TPL_DOCUMENT_URI`、`TPL_DOCUMENT_PATH`、`TPL_DOCUMENT_NAME`、`TPL_LANGUAGE`：有值时提供。

完整字段见 `examples/ctx.json`。所有变量共享一次时间和文档快照；同一个变量在模板中出现多次时只计算一次。`ctx.now.timezone` 是实际 UTC 偏移量，`ctx.document.language` 是 LSP 语言标识，例如 `markdown`。第一版没有语义信息来源，`ctx.symbol` 和 `ctx.selection` 为 `null`。

命令 stdout 是变量值，保留空格和末尾换行；stderr 用于日志。非零退出或超时会取消本次模板候选。默认超时 1000 毫秒；stdout 上限 64 KiB。Bash 用 `printf`、Python 用 `sys.stdout.write` 可以避免额外换行。

命令在生成补全时执行，可能因补全刷新执行多次，适合用来计算变量值。配置有变化时在下一次补全重新解析，配置错误保留上一次有效版本；删除配置文件后，在下一次补全清空活动模板，恢复文件后重新加载。脚本在下一次执行时直接读取。

## 验证

只保留一个端到端检查，直接与实际 LSP 服务通信：

```sh
cargo build -p server
python3 scripts/smoke.py
```

它检查多个用户缩写和最长匹配、Bash/Python 命令、中文和 emoji 的 UTF-16 替换范围、最终光标标记、文字转义，以及配置修改、删除和恢复。

用户已确认 Linux 和 macOS 的 Zed 手动功能验证通过；各检查项的详细结果、测试版本和对应提交尚未完整记录，见 [验证记录](doc/VERIFICATION.md)。发布前请在最终提交上重新确认开发扩展安装、模板展开和光标定位。

时间目前取自补全请求时刻。候选菜单停留跨分钟后才确认时，插入的时间可能仍是生成候选的时间；尚未实现确认时刷新。

接口依据：[Zed 扩展开发](https://zed.dev/docs/extensions/developing-extensions)、[语言服务器接入](https://zed.dev/docs/extensions/languages#language-servers)、[多语言服务器配置](https://zed.dev/docs/configuring-languages#choosing-language-servers)。

## 开源协议

本项目采用 [MIT 协议](LICENSE)。
