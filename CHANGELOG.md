# Changelog / 更新日志

All notable changes to this project will be documented in this file.
本项目的所有重要变更均记录在此文件中。

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-11-17

### 简体中文

- **统一聊天前缀配置**：支持通过 `configs/imomoe_chat_prefix/prefix.toml` 配置单个固定前缀或非空数组随机轮换前缀，未配置时使用通用默认前缀 `[Community]`。
- **CS2 色标编译与严格校验**：支持 16 种 CS2 命名聊天颜色标签及 UTF-8 文本，自动补齐前导空格、`\x01` 颜色复位与末尾分隔空格；强制单行、禁止控制字符、要求包含可见文本并限制编译后总长度不超过 96 字节。
- **热重载命令**：提供仅限服务端控制台与 RCON 执行的 `imomoe_chat_prefix_reload` 命令；重载失败时输出错误日志并保留上次有效配置。
- **Plugify 共享导出接口**：导出 `GetChatPrefix(out: string&) -> bool`，供其他 CS2 Plugify 插件作为可选依赖查询并复用当前前缀。
- **FFI 与生命周期安全**：为生命周期入口、命令回调与导出方法提供 `catch_unwind` 防护；开始卸载即停用前缀并阻止重载回调重新发布，若命令注销失败则保留注销状态供重试。

### English

- **Unified Chat Prefix Configuration**: Added `configs/imomoe_chat_prefix/prefix.toml` supporting either a single fixed prefix or an array of prefixes selected uniformly at random, falling back to `[Community]` when unconfigured.
- **CS2 Color Tag Compilation & Strict Validation**: Added support for 16 CS2 named chat color tags and UTF-8 text, automatically framing prefixes with leading space, `\x01` reset, and trailing separator space while enforcing single-line text, no control characters, visible content, and a 96-byte compiled budget.
- **Hot Reload Command**: Added `imomoe_chat_prefix_reload` restricted to server console and RCON; failed reloads log diagnostics and retain the last valid configuration.
- **Plugify Shared Export ABI**: Exported `GetChatPrefix(out: string&) -> bool` as an optional dependency interface for companion plugins.
- **FFI & Lifecycle Safety**: Guarded lifecycle, command, and export entry points with `catch_unwind`; disabled prefix emission immediately on unload and retained command-unregistration state if retry is needed.
