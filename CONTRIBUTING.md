# Contributing to Imomoe Chat Prefix

感谢对 Imomoe Chat Prefix 项目的关注与贡献。在提交代码或 Pull Request 之前，请阅读以下开发与协同规范。

---

## 开发环境与工具链

本项目基于 Rust 2024 Edition，绑定并依赖 [Plugify](https://github.com/untrustedmodders/plugify) 与 [S2SDK](https://github.com/untrustedmodders/plugify-plugin-s2sdk)。

- **Rust 工具链**：请确保通过 `rust-toolchain.toml` 使用固定的 `1.99.1` 版本，包含 `rustfmt` 与 `clippy` 组件：
  ```bash
  rustup toolchain install 1.99.1 --profile minimal --component rustfmt --component clippy
  ```
- **操作系统支持**：
  - Windows: x86_64 MSVC
  - Linux: x86_64 GNU（发布环境建议基于 Steam Runtime Sniper）

---

## 质量门禁与本地验证

所有提交必须通过以下四项标准门禁：

```bash
# 1. 格式化检查
cargo fmt --all -- --check

# 2. 静态检查（严格禁止告警）
cargo clippy --all-targets --locked -- -D warnings

# 3. 单元测试
cargo test --locked

# 4. 发布构建
cargo build --release --locked
```

---

## 核心设计与编码规范

1. **FFI 边界安全**：
   - 暴露给 Plugify 或 S2SDK 的外部 C ABI 入口（如 `start`、`end`、`GetChatPrefix` 与 `on_reload` 回调）必须使用 `catch_unwind` 拦截 panic，严禁 panic 跨越 C ABI 导致宿主进程崩溃。
   - 作为辅助型前缀服务，启动异常不得阻断 Plugify 宿主加载其他插件。

2. **权限与生命周期契约**：
   - `imomoe_chat_prefix_reload` 不允许客户端执行（不设置 `FCVAR_CLIENT_CAN_EXECUTE`，并在回调中校验服务端控制台上下文）。
   - 插件卸载（`end`）必须清空内存状态并精确注销已注册的控制台命令，防止悬挂回调。

3. **配置与兼容性契约**：
   - 配置解析采用严格模型（`#[serde(deny_unknown_fields)]`），仅接受经过完整校验的单行文本与合法 CS2 命名色标。
   - 重载时先解析与校验新配置，确认全部条目合法后再原子替换共享状态；任何错误均保留上一次有效配置。
   - 保持 `GetChatPrefix(out: string&) -> bool` ABI、前导空格、`0x01` 颜色复位与 96 字节上限等契约的向后兼容。

---

## 提交与 Pull Request 流程

1. 基于 `main` 分支拉取新的特性分支（如 `feat/my-feature` 或 `fix/my-bug`）。
2. 保持每次提交聚焦单一主题，提交说明简练准确；切勿在代码、测试或示例配置中提交真实社区名称、私有路径或服务端敏感信息。
3. 确保本地四门禁全部通过后发起 Pull Request。
4. PR 描述清晰说明改动动机、影响范围及验证方法（涉及 ABI 或热重载行为变更时建议附带测试说明）。
