<div align="center">

# Imomoe Chat Prefix (`imomoe_chat_prefix`)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Rust: 1.99.1](https://img.shields.io/badge/Rust-1.99.1%20(2024)-orange.svg)](rust-toolchain.toml)
[![CS2: Compatible](https://img.shields.io/badge/CS2-Dedicated%20Server-black.svg)](https://www.counter-strike.net/)
[![Platform: Plugify](https://img.shields.io/badge/Plugify-v3.1.0%2B-purple.svg)](https://github.com/untrustedmodders/plugify)
[![CI](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix/actions/workflows/ci.yml/badge.svg)](https://github.com/Imomoe-Community/Imomoe-Chat-Prefix/actions/workflows/ci.yml)

**A shared, configurable chat prefix Plugify plugin for Counter-Strike 2 dedicated servers.**

[双语 / Bilingual](README.md) · **English**

</div>

---

### Overview

**imomoe_chat_prefix** is an independent Rust [Plugify](https://github.com/untrustedmodders/plugify) plugin for Counter-Strike 2 dedicated servers that provides a centralized, customizable, and optionally randomized chat prefix to companion plugins. It does not intercept chat messages: callers choose which messages receive a prefix and how console output is formatted.

---

### Prerequisites

- CS2 Dedicated Server with compatible Plugify (`v3.1.0+`)
- Plugify Rust Module (`1.3.0` or `1.2.5`)
- S2SDK `>=3.4.0` (trimmed S2SDK providers must retain `AddConsoleCommand` and `RemoveCommand`)

---

### Installation

1. Build release binaries:

   ```bash
   cargo build --release --locked
   ```

2. Deploy [imomoe_chat_prefix.pplugin](imomoe_chat_prefix.pplugin) and the binary into `game/csgo/addons/plugify/envs/imomoe_chat_prefix/`:
   - **Windows**: `target/release/imomoe_chat_prefix.dll` → `bin/imomoe_chat_prefix.dll`
   - **Linux (Rust Module 1.3.0)**: `target/release/libimomoe_chat_prefix.so` → `bin/libimomoe_chat_prefix.so`
   - **Linux (Rust Module 1.2.5)**: rename to `bin/imomoe_chat_prefix.so`

3. Copy [prefix.example.toml](configs/imomoe_chat_prefix/prefix.example.toml) to `game/csgo/addons/plugify/configs/imomoe_chat_prefix/prefix.toml` and customize `prefix`.

4. Restart the server after installing, updating, or removing the plugin binary.

> [!NOTE]
> If `prefix.toml` is not present at startup, the built-in default `[Community]` is used. The repository ships only generic examples without community-specific presets.

---

### Configuration & Hot Reload

#### Fixed & Randomized Prefixes

`prefix.toml` uses strict TOML parsing (unknown fields rejected) and supports only `prefix`:

- **Fixed**: Set `prefix` to a single string:

  ```toml
  prefix = "{grey}[{gold}Community{grey}]"
  ```

- **Randomized**: Set `prefix` to a non-empty array of strings; one entry is chosen uniformly per emitted message (consecutive repeats allowed) and shared across all broadcast recipients:

  ```toml
  prefix = [
    "{grey}[{gold}Community{grey}]",
    "{blue}[{green}Community{blue}]",
    "{red}【{yellow}Community{red}】",
  ]
  ```

#### Supported Color Tags

Prefixes support UTF-8 text and CS2 named color tags (mapped in S2SDK chat control-byte order); a leading space, `{default}` color reset, and trailing separator space are appended automatically:

| Tag | Byte | Tag | Byte |
| :--- | :--- | :--- | :--- |
| `{default}` | `0x01` | `{yellow}` | `0x09` |
| `{darkred}` | `0x02` | `{bluegrey}` | `0x0A` |
| `{purple}` | `0x03` | `{blue}` | `0x0B` |
| `{green}` | `0x04` | `{darkblue}` | `0x0C` |
| `{olive}` | `0x05` | `{grey2}` | `0x0D` |
| `{lime}` | `0x06` | `{orchid}` | `0x0E` |
| `{red}` | `0x07` | `{lightred}` | `0x0F` |
| `{grey}` | `0x08` | `{gold}` | `0x10` |

#### Validation Rules

- **Single-line only**: Rejects newlines, Unicode line/paragraph separators (`U+2028` / `U+2029`), and any control characters.
- **Visible content**: Must contain visible non-whitespace characters after trimming, not just color tags.
- **Byte budget**: Compiled output including leading space, `0x01` reset, and trailing space must not exceed **96 bytes**.
- **Unsupported formats**: Rejects SourceMod configs, legacy numeric color codes, and RGB hex codes. Any invalid array element rejects the entire load/reload.

#### Runtime Reload

After editing `prefix.toml`, run from the **server console or RCON** (client execution is ignored):

```text
imomoe_chat_prefix_reload
```

- **Success**: Applies starting on the next emitted message without restarting the server or callers.
- **Failure**: Logs an error and retains the previous valid configuration.
- **Invalid config at startup**: Logs an error and temporarily disables shared prefix emission (`GetChatPrefix` returns `false`) until a valid reload succeeds.

---

### Plugify API & Integration

#### 1. Manifest & Symbol Import

Declare `imomoe_chat_prefix >=0.1.0` as an optional dependency in the caller's `.pplugin` manifest (see [imomoe_chat_prefix.pplugin](imomoe_chat_prefix.pplugin)):

```json
{
  "dependencies": [
    { "name": "imomoe_chat_prefix", "constraints": ">=0.1.0", "optional": true }
  ]
}
```

| Method | Description |
| :--- | :--- |
| `GetChatPrefix(out: string&) -> bool` | Copies one compiled CS2 chat prefix (including reset and separator) into `out`. Returns `false` when unavailable without altering `out`. |

Rust 2024 import snippet:

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

#### 2. Calling Contract & Dispatch Paths

- **Thread & Frequency**: Must be called on the game thread with a caller-allocated `plugify::Str`. Query **once per message** and reuse across all broadcast recipients.
- **Unavailable**: When the symbol is `None` or `GetChatPrefix` returns `false`, strip any placeholder marker and its adjacent space before sending.
- **`PrintToChat` path**: Split message lines before prepending, count prefix bytes toward message size limits, and never split on `\n` afterward because **`0x0A` is `{bluegrey}`**.
- **`ReplyToCommand` path**: For chat replies, trim the leading space and convert `0x0A` back to `{bluegrey}`; for console replies, omit color prefixes and strip placeholders plus `0x01`–`0x10` control bytes.

---

### Building & Development

Uses Rust 2024 Edition pinned to `1.99.1` ([rust-toolchain.toml](rust-toolchain.toml)). Run all four checks before submitting changes:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked
```

- [Contributing](CONTRIBUTING.md) · [Security](SECURITY.md) · [Changelog](CHANGELOG.md)

---

### License & Author

- **Author**: Mai Ooizumi ([@Mai-Ooizumi](https://github.com/Mai-Ooizumi))
- **License**: GNU Affero General Public License v3.0 or later ([AGPL-3.0-or-later](LICENSE))
