<div align="center">

# Imomoe Chat Prefix (`imomoe_chat_prefix`)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Rust: 1.99.1](https://img.shields.io/badge/Rust-1.99.1%20(2024)-orange.svg)](rust-toolchain.toml)
[![CS2: Compatible](https://img.shields.io/badge/CS2-Dedicated%20Server-black.svg)](https://www.counter-strike.net/)
[![Platform: Plugify](https://img.shields.io/badge/Plugify-v3.1.0%2B-purple.svg)](https://github.com/untrustedmodders/plugify)
[![CI](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix/actions/workflows/ci.yml/badge.svg)](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix/actions/workflows/ci.yml)

**面向 Counter-Strike 2 专用服务端的 Plugify 共享聊天前缀插件**<br>
<sub><sup>A shared, configurable chat prefix Plugify plugin for Counter-Strike 2 dedicated servers.</sup></sub>

**双语 / Bilingual** · [English](README.en.md)

</div>

---

### 项目简介 / Overview

**imomoe_chat_prefix** 是面向 Counter-Strike 2 (CS2) 专用服务端的独立 Rust [Plugify](https://github.com/untrustedmodders/plugify) 共享前缀插件。通过一份统一配置为接入的伴随插件提供可定制、可随机轮换的聊天前缀。本插件不拦截聊天消息：由接入方决定哪些提示使用前缀以及如何输出控制台文本。
<sub><sup>**imomoe_chat_prefix** is an independent Rust [Plugify](https://github.com/untrustedmodders/plugify) plugin for CS2 dedicated servers that supplies a unified, customizable, and optionally randomized chat prefix to companion plugins. It does not intercept chat messages; callers decide where to prepend the prefix and how to strip markers for console output.</sup></sub>

---

### 环境要求 / Prerequisites

- CS2 Dedicated Server 与兼容的 Plugify（`v3.1.0+`）<br><sub><sup>CS2 Dedicated Server with compatible Plugify (`v3.1.0+`)</sup></sub>
- Plugify Rust Module（`1.3.0` 或 `1.2.5`）<br><sub><sup>Plugify Rust Module (`1.3.0` or `1.2.5`)</sup></sub>
- S2SDK `>=3.4.0`（若使用裁剪过的 S2SDK provider，需保留 `AddConsoleCommand` 与 `RemoveCommand`）<br><sub><sup>S2SDK `>=3.4.0` (trimmed S2SDK providers must retain `AddConsoleCommand` and `RemoveCommand`)</sup></sub>

---

### 安装与部署 / Installation

1. 在项目根目录构建发布产物 / Build release binaries:

   ```bash
   cargo build --release --locked
   ```

2. 将 [imomoe_chat_prefix.pplugin](imomoe_chat_prefix.pplugin) 与编译产物安装到服务端 `game/csgo/addons/plugify/envs/imomoe_chat_prefix/` / Deploy manifest and binary:
   - **Windows**：将 `target/release/imomoe_chat_prefix.dll` 放入 `bin/imomoe_chat_prefix.dll`。
   - **Linux（Rust Module 1.3.0）**：将 `target/release/libimomoe_chat_prefix.so` 放入 `bin/libimomoe_chat_prefix.so`。
   - **Linux（Rust Module 1.2.5）**：将库重命名为 `bin/imomoe_chat_prefix.so`。

3. 将示例配置 [prefix.example.toml](configs/imomoe_chat_prefix/prefix.example.toml) 复制为 `game/csgo/addons/plugify/configs/imomoe_chat_prefix/prefix.toml`（若自定义过 Plugify 配置根目录，请放入实际 `configs/` 目录下），按需修改 `prefix`。<br><sub><sup>Copy `prefix.example.toml` to `game/csgo/addons/plugify/configs/imomoe_chat_prefix/prefix.toml` and customize `prefix`.</sup></sub>

4. 安装、更新或移除插件二进制后，重启服务端使插件生效。<br><sub><sup>Restart the server after installing, updating, or removing the plugin binary.</sup></sub>

> [!NOTE]
> 启动时若未找到 `prefix.toml`，插件会使用内置默认值 `[Community]`。仓库仅提供通用示例，不包含具体社区配置。<br><sub><sup>If `prefix.toml` is missing at startup, `[Community]` is used as the built-in default.</sup></sub>

---

### 配置与热重载 / Configuration & Hot Reload

#### 固定前缀与随机前缀 / Fixed & Randomized Prefixes

配置文件采用严格 TOML 解析（不允许未知字段），仅支持 `prefix` 字段 / Strictly parsed TOML supporting only `prefix`:

- **固定样式 / Fixed**：将 `prefix` 设为单个字符串：

  ```toml
  prefix = "{grey}[{gold}Community{grey}]"
  ```

- **随机样式 / Randomized**：将 `prefix` 设为非空字符串数组，每次发出带前缀的提示都会等概率抽取一条（允许连续抽中同一款）；同一条广播的所有接收者共用当次抽选结果：<br><sub><sup>An array of strings selects uniformly per emitted message and shares that selection across all broadcast recipients:</sup></sub>

  ```toml
  prefix = [
    "{grey}[{gold}Community{grey}]",
    "{blue}[{green}Community{blue}]",
    "{red}【{yellow}Community{red}】",
  ]
  ```

#### 支持的颜色标签 / Supported Color Tags

前缀使用 CS2 命名色标（按 S2SDK 聊天控制字节顺序映射），支持 UTF-8 文本；正文前的分隔空格与 `{default}` 颜色复位由插件自动附加：<br><sub><sup>Supports UTF-8 text and CS2 named color tags; leading space, `{default}` reset, and trailing separator space are appended automatically:</sup></sub>

| 色标 / Tag | 控制字节 / Byte | 色标 / Tag | 控制字节 / Byte |
| :--- | :--- | :--- | :--- |
| `{default}` | `0x01` | `{yellow}` | `0x09` |
| `{darkred}` | `0x02` | `{bluegrey}` | `0x0A` |
| `{purple}` | `0x03` | `{blue}` | `0x0B` |
| `{green}` | `0x04` | `{darkblue}` | `0x0C` |
| `{olive}` | `0x05` | `{grey2}` | `0x0D` |
| `{lime}` | `0x06` | `{orchid}` | `0x0E` |
| `{red}` | `0x07` | `{lightred}` | `0x0F` |
| `{grey}` | `0x08` | `{gold}` | `0x10` |

#### 校验规则 / Validation Rules

- **单行限制 / Single-line only**：禁止换行符、Unicode 行分隔符（`U+2028` / `U+2029`）及任何控制字符。<br><sub><sup>Forbids newlines, Unicode line/paragraph separators, and control characters.</sup></sub>
- **可见内容 / Visible content**：去除首尾空白后必须包含可见非空白字符，不能只填写色标。<br><sub><sup>Must contain non-whitespace visible text beyond color tags.</sup></sub>
- **字节预算 / Byte budget**：编译为聊天控制字节并加上前导空格、`0x01` 复位与末尾空格后，总长度不得超过 **96 字节**。<br><sub><sup>Compiled output including framing space and `0x01` reset must not exceed 96 bytes.</sup></sub>
- **不支持的格式 / Unsupported formats**：不读取 SourceMod 配置、旧数字色标或 RGB 十六进制颜色代码；数组中任意一条校验失败都会拒绝整次加载或重载。<br><sub><sup>Legacy SourceMod/numeric/hex codes are rejected; any invalid entry rejects the whole load/reload.</sup></sub>

#### 运行时重载 / Runtime Reload

修改 `prefix.toml` 后，在**服务端控制台或 RCON** 执行（客户端执行会被忽略）/ Run from server console or RCON:

```text
imomoe_chat_prefix_reload
```

- **重载成功 / Success**：从下一条消息开始生效，无需重启服务端或调用方插件。<br><sub><sup>Takes effect on the next message without restarting.</sup></sub>
- **重载失败 / Failure**：输出错误日志并保留上一次有效配置。<br><sub><sup>Logs an error and retains the previous valid configuration.</sup></sub>
- **启动时配置文件存在但无效 / Invalid on startup**：记录错误并暂时停用共享前缀（`GetChatPrefix` 返回 `false`），直至热重载成功。<br><sub><sup>Logs an error and temporarily disables prefix emission (`GetChatPrefix` returns `false`) until a valid reload succeeds.</sup></sub>

---

### Plugify API 与其他插件接入 / Plugify API & Integration

#### 1. 声明依赖与导入符号 / Manifest & Symbol Import

在调用方 `.pplugin` 清单中将 `imomoe_chat_prefix >=0.1.0` 声明为可选依赖（见 [imomoe_chat_prefix.pplugin](imomoe_chat_prefix.pplugin)）：<br><sub><sup>Declare `imomoe_chat_prefix >=0.1.0` as an optional dependency in your `.pplugin` manifest:</sup></sub>

```json
{
  "dependencies": [
    { "name": "imomoe_chat_prefix", "constraints": ">=0.1.0", "optional": true }
  ]
}
```

| 方法 / <sub><sup>Method</sup></sub> | 说明 / <sub><sup>Description</sup></sub> |
| :--- | :--- |
| `GetChatPrefix(out: string&) -> bool` | 将选中且编译好的 CS2 聊天前缀写入 `out`（含颜色复位与分隔空格）；不可用时返回 `false` 并保持 `out` 不变。<br><sub><sup>Copies one compiled CS2 chat prefix into `out`; returns `false` when unavailable without modifying `out`.</sup></sub> |

在 Rust 2024 插件中声明导入符号并通过 `addr_of!` 读取 / Rust 2024 FFI import example:

```rust
type GetChatPrefix = unsafe extern "C" fn(&mut plugify::Str) -> bool;

#[allow(non_upper_case_globals)]
#[unsafe(no_mangle)]
static mut __imomoe_chat_prefix_GetChatPrefix: Option<GetChatPrefix> = None;

fn fetch_prefix() -> Option<String> {
    let get = unsafe { std::ptr::addr_of!(__imomoe_chat_prefix_GetChatPrefix).read() }?;
    let mut out = plugify::Str::new();
    unsafe { get(&mut out) }.then(|| out.as_str().to_owned())
}
```

#### 2. 调用契约与消息发送路径 / Calling Contract & Dispatch Paths

- **线程与调用频率 / Thread & Frequency**：必须在游戏主线程查询；**每条消息只查询一次**，同一轮广播复用返回值。<br><sub><sup>Query on the game thread once per logical message and reuse across all broadcast recipients.</sup></sub>
- **前缀不可用 / Unavailable**：当导入符号为 `None` 或返回 `false` 时，移除占位符及紧邻的一个分隔空格再发送正文。<br><sub><sup>When `None` or `false`, strip any placeholder marker and its adjacent space before sending.</sup></sub>
- **`PrintToChat` 路径**：先拆好正文行再拼接返回值，并将前缀字节计入长度预算；注意 **`0x0A` 是 `{bluegrey}` 控制码**，拼接后不能按 `\n` 拆行。<br><sub><sup>Split lines before prepending because `0x0A` is the `{bluegrey}` control byte, not a newline.</sup></sub>
- **`ReplyToCommand` 路径**：聊天回复需去掉返回值开头空格并将 `0x0A` 还原为 `{bluegrey}`；控制台回复不注入聊天颜色前缀，并清除占位符及 `0x01`–`0x10` 控制色符。<br><sub><sup>For chat replies trim the leading space and convert `0x0A` back to `{bluegrey}`; for console replies strip placeholders and `0x01`–`0x10` control bytes.</sup></sub>

---

### 构建与开发 / Building & Development

本仓库使用 Rust 2024 Edition，工具链固定为 `1.99.1`（见 [rust-toolchain.toml](rust-toolchain.toml)）。提交改动前运行标准四门禁：<br><sub><sup>Uses Rust 2024 Edition pinned to `1.99.1`. Run the four standard checks before submitting changes:</sup></sub>

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked
```

- [贡献指南 / Contributing](CONTRIBUTING.md) · [安全通告 / Security](SECURITY.md) · [版本记录 / Changelog](CHANGELOG.md)

---

### 开源协议与作者 / License & Author

- **作者 / Author**：Mai Ooizumi ([@Mai-Ooizumi](https://github.com/Mai-Ooizumi))
- **开源协议 / License**：GNU Affero General Public License v3.0 or later ([AGPL-3.0-or-later](LICENSE))
