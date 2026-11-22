# Contributing Guide / 贡献指南

感谢对 **Imomoe Chat Prefix** 项目的关注与贡献！在提交代码或 Pull Request 之前，请阅读以下开发与协同规范。

Thank you for contributing to **Imomoe Chat Prefix**! Please review these guidelines before submitting code or a Pull Request.

---

## 1. 开发环境与工具链 / Environment & Toolchain

本项目基于 Rust 2024 Edition，绑定并依赖 [Plugify](https://github.com/untrustedmodders/plugify) 与 [S2SDK](https://github.com/untrustedmodders/plugify-plugin-s2sdk)。

Built with Rust 2024 Edition against [Plugify](https://github.com/untrustedmodders/plugify) and [S2SDK](https://github.com/untrustedmodders/plugify-plugin-s2sdk):

- **Rust 工具链 / Toolchain**：请通过 `rust-toolchain.toml` 使用固定的 `1.99.1` 版本（含 `rustfmt` 与 `clippy`）：
  ```bash
  rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
  ```
- **操作系统支持 / Supported OS**：Windows x86_64 MSVC、Linux x86_64 GNU（发布环境建议基于 Steam Runtime Sniper）。

---

## 2. 质量门禁与本地验证 / Quality Gates

所有提交必须通过以下四项标准门禁 / Every submission must pass all four checks:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --locked -- -D warnings
cargo test --locked
cargo build --release --locked
```

---

## 3. 核心设计与编码规范 / Architecture & Safety Contracts

1. **FFI 边界安全 / FFI Boundary Safety**：
   - 暴露给 Plugify 或 S2SDK 的外部 C ABI 入口（`start`、`end`、`GetChatPrefix` 与 `on_reload`）必须使用 `catch_unwind` 拦截 panic，且启动异常不得阻断其他插件加载。
   - External C ABI entry points must catch panics with `catch_unwind`, and startup errors must never block loading of other plugins.
2. **权限与生命周期契约 / Privilege & Lifecycle**：
   - `imomoe_chat_prefix_reload` 不允许客户端执行（不设 `FCVAR_CLIENT_CAN_EXECUTE`，并在回调校验服务端控制台上下文）；卸载（`end`）清空状态并精确注销命令。
   - Keep `imomoe_chat_prefix_reload` server/RCON-only; clear state and unregister console commands cleanly on `end`.
3. **配置与兼容性契约 / Config & ABI Compatibility**：
   - 采用严格 TOML 解析（`#[serde(deny_unknown_fields)]`），仅在全部条目通过校验后原子替换共享状态；保持 `GetChatPrefix(out: string&) -> bool` 与 96 字节上限契约向后兼容。
   - Use strict TOML parsing, atomic reload swaps, and backward-compatible `GetChatPrefix` / 96-byte budget contracts.

---

## 4. 提交与 PR 流程 / Pull Request Workflow

1. 基于 `main` 分支拉取特性分支并保持每次提交聚焦单一主题，切勿提交真实社区名称、私有路径或服务端敏感信息。
   Branch from `main`, keep commits focused, and never commit real community names, private paths, or secrets.
2. 确保本地四门禁全部通过后发起 Pull Request，并清晰说明改动动机、影响范围及验证方法。
   Open a PR after all four local checks pass, summarizing motivation, scope, and verification.
