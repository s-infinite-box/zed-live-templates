# 平台测试记录

[English](TESTING.md) | 简体中文

## 官方 Zed 手动验证

2026-10-06，用户明确补充确认：Linux 和 macOS 的官方 Zed 版本也已完成手动功能测试，均通过。本文以官方 Zed 的测试结果作为主要兼容性依据；此前 ZedG 的环境观察和日志作为补充测试记录保留。

| 平台 | 编辑器 | 用户确认结果 |
|---|---|---|
| Linux x86_64 | 官方 Zed | 手动功能验证通过 |
| macOS ARM64 | 官方 Zed | 手动功能验证通过 |

此前直接观察到的 Mac 官方安装版本为 Zed 1.22.0；两端官方 Zed 手测的准确版本、日期和扩展提交没有单独提供。下方补充记录中的具体版本和提交对应原始运行记录，不直接改写为官方 Zed 测试的版本和提交。

## 补充记录：Linux 当前环境与已安装服务

2026-10-06 直接检查当前 Linux 环境，得到：

| 项目 | 本次观察 |
|---|---|
| 系统 | Fedora Linux 44，KDE Plasma Desktop Edition |
| 内核 / 架构 | `7.2.8-200.fc44.x86_64`，x86_64 |
| 编辑器 | 桌面入口为 ZedG；启动日志版本为 `1.22.0+stable.364.76659a55a8c10ed355a070f8764a0b1733e3c115` |
| 发布检查使用的 Rust / Cargo | 1.96.0 |
| Python | 3.14.7 |
| 开发扩展登记 | 仍为 `live-templates`，链接到本项目目录 |
| 语言服务器名称 | `templates` |
| 已配置的原生服务 | 旧扩展工作目录中的 ELF 64-bit x86_64 可执行文件 |
| 检查时仓库提交 | `518a0d04a55909bb0c5e7d4c2854e20856b1fd91` |

CLI 的版本显示未包含版本号，但给出了上述 SHA；版本号来自当前应用启动日志，ZedG 名称来自桌面入口。ZedG 是提供简体中文界面的 Zed 构建。

用户此前已确认 Linux 手动功能验证通过，其准确测试日期和被测扩展提交未记录。本表记录的是当前观察环境与当前仓库提交，不能据此把历史手测认定为针对 `518a0d0` 的测试。

本次使用发布准备期间的 LSP 端到端脚本直接检查 Zed 设置中配置的已安装 `server`，`dt`、`todo`、`ctx`、`@dt`、Bash/Python 变量、中文和 emoji 的 UTF-16 替换范围、光标标记、转义，以及配置修改、删除、恢复均通过。没有替换该服务二进制；这项协议检查不等于重新进行了界面操作。该测试脚本随后已按用户要求从仓库移除，本文保留已完成的历史结果。

## 补充记录：macOS ZedG 测试过程

### 环境与结论

2026-10-06，在 macOS Apple Silicon 上完成原生服务构建、LSP 端到端检查和 ZedG 界面验证，全部通过。界面操作由用户执行并确认结果；构建、协议检查和服务启动日志由命令行核实。

| 项目 | 本次环境 |
|---|---|
| macOS | 26.6.2，Darwin 25.6.0，ARM64 |
| Rust / Cargo | 1.96.0 |
| Python | 3.14.4，已有虚拟环境 |
| 编辑器 | ZedG v1.22.0 |
| 被测代码 | `0c863306f829000bef41b7824fa5307b90e6321f` |
| 被测扩展 ID | `live-templates`；后续发布准备改名为 `live-templates-lsp` |
| 模板配置 | `examples/templates.toml`；端到端脚本会在临时副本中增加测试模板 |

### 自动检查

原生服务使用现有 Rust `stable` 工具链构建，其版本为 1.96.0。构建从父项目目录以外执行，通过 `--manifest-path` 指定项目，避免父项目 Cargo 镜像配置影响依赖解析。本次离线复查使用以下命令：

```sh
project_dir="/absolute/path/to/zed-live-templates"
cargo +stable build --manifest-path "$project_dir/Cargo.toml" \
  -p server --release --locked --offline
file "$project_dir/target/release/server"
```

构建成功，产物为 `Mach-O 64-bit executable arm64`。`--offline` 要求依赖已缓存，首次构建可以去掉该选项。

使用已有 Python 环境，通过发布准备期间的端到端脚本与 `target/release/server` 进行 LSP 标准输入输出通信。该脚本随后已从仓库移除，下表为已完成的历史检查结果：

| 检查内容 | 结果 |
|---|---|
| `dt` 日期模板、`todo` 待办模板 | 通过 |
| Bash 中文星期、主机名脚本 | 通过 |
| Python 读取 JSON 上下文，变量输出中的 `$` 和 `}` 转义 | 通过 |
| `记录🙂 dt` 的 UTF-16 替换范围 | 通过，只替换缩写 |
| `$END$` 转换为 snippet 的 `$0` | 通过 |
| `dt` 与 `@dt` 重叠时优先匹配最长缩写 | 通过 |
| 配置修改、删除、恢复相同内容后重新加载 | 通过 |
| LSP 正常关闭、进程退出 | 通过 |

脚本最终输出：

```text
OK: dt / todo / ctx / @dt, Bash / Python, UTF-16 range, cursor, snippet escaping, configuration reload / removal / restoration
```

### 界面验证

在独立数据目录中加载 `Live Templates` 开发扩展，使用项目级 `.zed/settings.json` 指定 `templates` 语言服务器、Mac 原生 `server` 和模板配置绝对路径。测试文件为 Markdown。

| 用户操作 | 预期与实际结果 |
|---|---|
| 逐字输入 `dt` | 自动出现模板候选，通过 |
| 选中 `dt` 候选并按 Enter | 缩写替换为中文星期、当前日期、时间和主机名，通过 |
| 在展开位置输入文字 | 光标位于标题与 `---` 之间的空行，通过 |
| 输入 `todo` 并确认候选 | 展开为 `- [ ] `，光标在末尾，通过 |
| 输入 `记录🙂 dt` 并展开 | 前缀 `记录🙂 ` 保留，只替换 `dt`，通过 |
| 在新行再次输入 `dt` | 仍能弹出候选并展开，通过 |

用户最终确认“测试全部 ok”。

### 启动问题与处理

本次首次输入 `dt` 没有反应，排查发现两处接入问题：

1. **扩展目录未被使用。** macOS 普通 CLI 启动应用时，没有把 `--user-data-dir` 传给应用进程。测试文件打开在主实例中，主实例加载原有的 28 个扩展，没有 `Live Templates`；独立数据目录当时没有运行产物。改为直接启动应用主程序后，独立实例加载了模板扩展。
2. **项目尚未信任。** 新实例日志显示 `Waiting for worktree ... to be trusted, before starting language server templates`。用户信任该测试项目后，日志确认 `templates` 启动，原生服务进程运行，`dt` 随后出现候选。

macOS 隔离验证可以直接启动应用主程序，确保扩展安装目录与应用实际使用的数据目录一致：

```sh
project_dir="/absolute/path/to/zed-live-templates"
data_dir="$project_dir/.tmp/zed-data"
/Applications/ZedG.app/Contents/MacOS/zed \
  --user-data-dir "$data_dir" "$project_dir"
```

本次数据目录已预先登记开发扩展；新建空目录时，需要在该实例中安装扩展并按 [中文 README](../README.zh-CN.md) 配置语言服务器。使用官方 Zed 时，应用路径换成 `/Applications/Zed.app/Contents/MacOS/zed`。普通 `--new` 只表示新建工作区，不代表新进程或独立数据目录。

项目级配置首次使用时需要信任项目。若没有弹窗，点击标题栏的感叹号，或执行 `workspace: toggle worktree security`。没有候选时先检查扩展和服务是否加载，再执行 `editor: show completions`；日志入口为 `zed: open log`。

相关说明：[Zed CLI 源码](https://github.com/zed-industries/zed/blob/76659a55a8c10ed355a070f8764a0b1733e3c115/crates/cli/src/main.rs)、[项目信任](https://zed.dev/docs/worktree-trust)、[开发扩展](https://zed.dev/docs/extensions/developing-extensions)。

### 验证范围

本次确认了 macOS ARM64 原生服务、动态 Bash/Python 变量、WASM 扩展加载，以及 ZedG v1.22.0 中的模板补全、展开和光标定位。

- 本次使用已在 Linux 构建的 `extension.wasm`，在 Mac 重新构建原生 `server`；未验证 Mac 本机重新编译 WASM 或 Zed 自动构建开发扩展的流程。
- 本节详细界面记录来自 ZedG 1.22.0；官方 Zed 的首次启动问题属于当时的接入问题。用户已补充确认官方 Zed 测试成功，主要结果见上方“官方 Zed 手动验证”。
- 本次没有验证 macOS Intel、其他编辑器版本或 Markdown 以外的语言。
- Mac 的界面被测提交使用旧扩展 ID `live-templates`；本次直接检查也确认两端登记仍为旧 ID。后续改名和发布准备的最终提交检查见 [验证记录](VERIFICATION.md)。
- 日期和时间取自候选生成时刻；候选菜单跨分钟停留后确认时，时间可能仍是生成候选时的值。
