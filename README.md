# Imomoe Chat Prefix

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Rust: 1.99.1](https://img.shields.io/badge/Rust-1.99.1%20(2024)-orange.svg)](rust-toolchain.toml)
[![CS2: Compatible](https://img.shields.io/badge/CS2-Dedicated%20Server-black.svg)](https://www.counter-strike.net/)
[![Platform: Plugify](https://img.shields.io/badge/Plugify-v3.1.0%2B-purple.svg)](https://github.com/untrustedmodders/plugify)
[![CI](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix/actions/workflows/ci.yml/badge.svg)](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix/actions/workflows/ci.yml)

面向 CS2 Dedicated Server 的 Plugify 共享聊天前缀插件。通过一份统一配置为接入的第三方插件提供可定制、可随机轮换的聊天前缀。本插件不拦截消息：由接入方选择哪些提示使用前缀，以及如何输出控制台文本。

---

## 环境要求

- CS2 Dedicated Server 与兼容的 Plugify（v3.1.0+）
- Plugify Rust Module（1.3.0 或 1.2.5）
- S2SDK `>=3.4.0`（若使用裁剪过的 S2SDK provider，需保留 `AddConsoleCommand` 与 `RemoveCommand`）

---

## 安装与部署

1. 在项目根目录构建发布产物：

   ```bash
   cargo build --release --locked
   ```

2. 将 [imomoe_chat_prefix.pplugin](imomoe_chat_prefix.pplugin) 与编译产物安装到服务端插件目录 `game/csgo/addons/plugify/envs/imomoe_chat_prefix/`：
   - **Windows**：将 `target/release/imomoe_chat_prefix.dll` 放入该目录的 `bin/imomoe_chat_prefix.dll`。
   - **Linux（Rust Module 1.3.0）**：将 `target/release/libimomoe_chat_prefix.so` 放入 `bin/libimomoe_chat_prefix.so`。
   - **Linux（Rust Module 1.2.5）**：将库重命名为 `bin/imomoe_chat_prefix.so`。

3. 将示例配置 [prefix.example.toml](configs/imomoe_chat_prefix/prefix.example.toml) 复制为 `game/csgo/addons/plugify/configs/imomoe_chat_prefix/prefix.toml`（若自定义过 Plugify 配置根目录，请放入实际 `configs/` 目录下），按需修改 `prefix`。

4. 安装、更新或移除插件二进制后，重启服务端使插件生效。

> [!NOTE]
> 启动时若未找到 `prefix.toml`，插件会使用内置默认值 `[Community]`。仓库仅提供通用示例，不包含具体社区配置。

---

## 配置与热重载

### 固定前缀与随机前缀

配置文件采用严格 TOML 解析（不允许未知字段），仅支持 `prefix` 字段：

- **固定样式**：将 `prefix` 设为单个字符串：

  ```toml
  prefix = "{grey}[{gold}Community{grey}]"
  ```

- **随机样式**：将 `prefix` 设为非空字符串数组，每次发出带前缀的提示都会等概率抽取一条（允许连续抽中同一款）；同一条广播的所有接收者共用当次抽选结果：

  ```toml
  prefix = [
    "{grey}[{gold}Community{grey}]",
    "{blue}[{green}Community{blue}]",
    "{red}【{yellow}Community{red}】",
  ]
  ```

### 支持的颜色标签

前缀使用 CS2 命名色标（按 S2SDK 聊天控制字节顺序映射），支持 UTF-8 文本；正文前的分隔空格与 `{default}` 颜色复位由插件自动附加：

| 色标 | 控制字节 | 色标 | 控制字节 |
| --- | --- | --- | --- |
| `{default}` | `0x01` | `{yellow}` | `0x09` |
| `{darkred}` | `0x02` | `{bluegrey}` | `0x0A` |
| `{purple}` | `0x03` | `{blue}` | `0x0B` |
| `{green}` | `0x04` | `{darkblue}` | `0x0C` |
| `{olive}` | `0x05` | `{grey2}` | `0x0D` |
| `{lime}` | `0x06` | `{orchid}` | `0x0E` |
| `{red}` | `0x07` | `{lightred}` | `0x0F` |
| `{grey}` | `0x08` | `{gold}` | `0x10` |

### 校验规则

- **单行限制**：禁止换行符、Unicode 行分隔符（`U+2028` / `U+2029`）及任何控制字符。
- **可见内容**：去除首尾空白后必须包含可见非空白字符，不能只填写色标。
- **字节预算**：编译为聊天控制字节并加上前导空格、`0x01` 复位与末尾空格后，总长度不得超过 **96 字节**（为调用方消息正文预留空间）。
- **不支持的格式**：不读取 SourceMod 配置、旧数字色标或 RGB 十六进制颜色代码；数组中任意一条校验失败都会拒绝整次加载或重载。

### 运行时重载

修改 `prefix.toml` 后，在**服务端控制台或 RCON** 执行（客户端执行会被忽略）：

```text
imomoe_chat_prefix_reload
```

- **重载成功**：从下一条消息开始生效，无需重启服务端或调用方插件。
- **重载失败**（文件缺失、语法错误或校验失败）：输出错误日志并保留上一次有效配置。
- **启动时配置文件存在但无效**：记录错误并暂时停用共享前缀（`GetChatPrefix` 返回 `false`），直至热重载成功。

---

## 其他插件接入

### 1. 声明依赖与导入符号

在调用方插件的 `.pplugin` 清单中将 `imomoe_chat_prefix >=0.1.0` 声明为可选依赖（接口定义以 [imomoe_chat_prefix.pplugin](imomoe_chat_prefix.pplugin) 为准）：

```json
{
  "dependencies": [
    { "name": "imomoe_chat_prefix", "constraints": ">=0.1.0", "optional": true }
  ]
}
```

导出方法签名：

```text
imomoe_chat_prefix.GetChatPrefix(out: string&) -> bool
```

在 Rust 插件中声明对应导入符号（由 Plugify Rust Module 自动填充），并在 Rust 2024 中通过 `addr_of!` 读取：

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

### 2. 调用契约与消息发送路径

- **线程与调用频率**：必须在游戏主线程传入调用方创建的 `plugify::Str` 进行查询。**每条消息只查询一次**：广播发送时在同一轮内复用返回值（即使正文按接收者语言本地化），下一条消息再重新查询。
- **前缀不可用**：当导入符号为 `None` 或 `GetChatPrefix` 返回 `false` 时，`out` 保持不变。使用可配置占位符的调用方应移除占位符及其紧邻的一个分隔空格，再发送正文。
- **`PrintToChat` 路径**：先拆好正文行，再将返回值直接拼接在正文前，并将前缀字节计入消息长度预算。返回值已包含前导空格、CS2 颜色控制字节、`\x01` 默认色复位及末尾分隔空格。注意 **`0x0A` 是 `{bluegrey}` 控制码**，拼接前缀后不能再按 `\n` 拆分换行。
- **`ReplyToCommand` 路径**：
  - **聊天回复**：去掉返回值开头的空格（SDK 会自动补空格），并将前缀中的 `0x0A` 替换回 `{bluegrey}`（否则 SDK 会将原始 LF 解释为换行）。
  - **控制台回复**：不注入聊天颜色前缀；去掉消息中的占位符及 CS2 控制色符 `0x01`–`0x10`。

---

## 构建与开发

本仓库使用 Rust 2024 Edition，工具链固定为 `1.99.1`（见 [rust-toolchain.toml](rust-toolchain.toml)）。提交改动前运行标准四门禁：

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked
```

- [贡献指南](CONTRIBUTING.md) · [安全通告](SECURITY.md) · [版本记录](CHANGELOG.md)

---

## 许可证

[GNU Affero General Public License v3.0 or later (AGPL-3.0-or-later)](LICENSE)。
