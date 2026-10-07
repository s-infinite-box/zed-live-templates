# Zed 动态模板

[English](README.md) | 简体中文

在 Markdown 中输入自己配置的缩写，选择补全候选后确认，插入模板并定位到 `$END$`。支持日期、时间、机器名和可选的命令变量。模板和缩写由用户配置，`dt` 和 `todo` 只是示例。

扩展 ID 为 `live-templates-lsp`，Zed 设置中的语言服务器名称为 `templates`。扩展目前**只注册 Markdown**。

## 安装和配置

官方仓库收录后，在 Zed 扩展列表安装 **Live Templates**。收录前可以按下文的开发安装方式使用。

首次使用时，扩展从本项目的 [GitHub Releases](https://github.com/s-infinite-box/zed-live-templates/releases) 自动下载对应系统、CPU 架构和扩展版本的预编译服务。Zed 自动启动它，并缓存在扩展工作目录。**普通用户无需安装 Rust，也无需填写服务程序路径。** 首次下载需要访问 GitHub 发布附件，后续启动复用缓存，可离线使用。升级扩展后使用匹配的新版本；下载失败后在下次启动语言服务器时重试。

预编译服务覆盖六种组合：

| 系统 | x86_64 | ARM64 |
| --- | --- | --- |
| Linux | 静态 musl | 静态 musl |
| macOS | Intel | Apple Silicon |
| Windows | MSVC | MSVC |

根据下面的示例创建 `templates.toml`。默认位置为：

| 平台 | 默认模板配置路径 |
| --- | --- |
| Linux / macOS | `~/.config/zed-live-templates/templates.toml` |
| Windows | `%APPDATA%\zed-live-templates\templates.toml` |
| 设置了 `XDG_CONFIG_HOME` 的任意系统 | `$XDG_CONFIG_HOME/zed-live-templates/templates.toml` |

如果现有 Zed 设置限制了 Markdown 的语言服务器，将 `"templates"` 加到该列表。使用其他模板文件时，可以合并以下设置，仅替换模板文件的绝对路径。Windows JSON 路径使用双反斜杠或正斜杠：

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

`"..."` 保留其他语言服务器。打开 Markdown 文件，输入已配置的缩写并确认模板候选。没有自动弹出菜单时执行 `editor: show completions`。项目设置需要先信任工作区；没有模板配置时不提供候选。

### 可选的服务覆盖配置

高级用户可以设置 `lsp.templates.binary.path` 为本机服务程序的绝对路径，此时完全跳过下载。`binary.arguments` 和 `binary.env` 在自动安装和覆盖路径两种情况下均生效。通过参数指定的 `--config` 优先于 `initialization_options.config_path`：

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

安装过旧 ID `live-templates` 的开发扩展时，先卸载旧扩展再安装 `live-templates-lsp`。模板文件和 `lsp.templates` 设置可继续使用；删除 `binary.path` 即切换为自动安装。

## 开发安装

只有从源码构建才需要 Rust。构建及发布流程已通过 Rust 1.90.0 验证，也已通过 1.96.0 开发构建。安装 `wasm32-wasip2` 目标后，在 Zed 命令面板执行 `zed: install dev extension`，选择本仓库目录。开发扩展默认也会下载已发布的服务；验证原生服务源码改动时，本机构建后使用可选覆盖配置：

```sh
cargo build -p server --release --locked
cargo build -p extension --release --target wasm32-wasip2 --locked
cargo test --workspace --locked
```

原生输出为 `target/release/server`，Windows 下为 `server.exe`；WASM 输出为 `target/wasm32-wasip2/release/extension.wasm`。官方插件包只含清单和 WASM，原生服务由 [.github/workflows/server.yml](.github/workflows/server.yml) 构建并发布到 GitHub Releases。

## 配置模板

```toml
version = 1

[formats]
date = "%Y/%-m/%-d"
time = "%H:%M"
timezone = "local"

[[templates]]
id = "daily_note"
trigger = "dt"
description = "插入日期记录"
languages = ["Markdown"]
body = '''# $date$ $time$ $host$
$END$
---'''

[[templates]]
id = "task"
trigger = "todo"
languages = ["Markdown"]
body = '- [ ] $END$'
```

内置变量只有 `$date$`、`$time$`、`$host$`。`$END$` 表示最终光标位置，省略时光标在末尾。星期等额外格式可用用户脚本实现，插件提供原始星期编号供脚本转换。`timezone` 支持 `local` 和 `UTC`。

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

用户已确认 Linux 和 macOS 的官方 Zed 手动功能验证通过；Linux 和 macOS 的新 ID `live-templates-lsp` 均已完成官方 Zed 1.22.0 复测。发布准备期间的 LSP 端到端检查已通过，测试脚本随后已从仓库移除，历史结果保留在 [测试记录](doc/TESTING.zh-CN.md)。被测源码、构建与发布检查见 [验证记录](doc/VERIFICATION.md)。

时间目前取自补全请求时刻。候选菜单停留跨分钟后才确认时，插入的时间可能仍是生成候选的时间；尚未实现确认时刷新。

接口依据：[Zed 扩展开发](https://zed.dev/docs/extensions/developing-extensions)、[语言服务器接入](https://zed.dev/docs/extensions/languages#language-servers)、[多语言服务器配置](https://zed.dev/docs/configuring-languages#choosing-language-servers)。

## 开源协议

本项目采用 [MIT 协议](LICENSE)。

内置变量不需要额外解释器；自定义命令所用的 Bash、Python 或其他程序需在本机可用。Windows 超时会结束直接启动的命令进程，命令后代进程的清理目前仅在 Unix 平台保证。
