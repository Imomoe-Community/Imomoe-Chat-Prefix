# Security Policy / 安全策略

## 支持的版本 / Supported Versions

当前仅对最新版本及 `main` 分支的持续演进提供安全修复支持。

Security fixes are provided for the latest release and active `main` branch development:

| 版本 / Version | 是否支持 / Supported |
| :--- | :--- |
| 0.1.x | :white_check_mark: |
| < 0.1.0 | :x: |

---

## 报告安全问题 / Reporting a Vulnerability

如果您在 **Imomoe Chat Prefix** 中发现了潜在的安全漏洞或稳定性风险（如跨 FFI 边界内存安全问题、权限校验绕过导致的未授权重载、异常配置触发的服务端崩溃等），请**不要**通过公开 Issue 提交。

If you discover a potential vulnerability or stability risk in **Imomoe Chat Prefix** (such as FFI memory safety bugs, unauthorized reload bypasses, or crashes from malformed configs), please **do not** disclose details in a public GitHub issue.

请通过以下方式联系维护者并提供复现信息：
Please contact the maintainer privately with reproduction details:

- **GitHub Private Vulnerability Reporting**：在仓库 Security 选项卡发起私密漏洞报告 / submit a private report via the repository Security tab。
- **Maintainer**：联系 [Mai Ooizumi](https://github.com/Mai-Ooizumi)。

### 提报内容建议 / Recommended Details

1. 插件版本、commit 哈希以及所依赖的 S2SDK / Plugify 版本。
   Plugin version, commit hash, and S2SDK / Plugify versions.
2. 操作系统与服务端架构（Windows / Linux）。
   OS and server architecture (Windows / Linux).
3. 最小脱敏复现配置（`prefix.toml` 片段）与复现步骤。
   Minimal sanitized `prefix.toml` snippet and reproduction steps.
4. 异常堆栈信息或服务端控制台日志（请先剔除密钥、Token 或私有信息）。
   Stack traces or console logs with secrets and private details removed.

---

## 响应流程 / Response Process

1. **确认与评估 / Triage**：维护者将在 48 小时内确认并评估漏洞严重程度 / Maintainer acknowledges and triages within 48 hours.
2. **修复与验证 / Fix & Verify**：在私有分支完成修复并通过单元与实机验证 / Patched privately and verified with unit and server tests.
3. **发版与披露 / Release & Disclosure**：发布修复版本并在安全通告中致谢报告者 / Release fix and publish advisory.
