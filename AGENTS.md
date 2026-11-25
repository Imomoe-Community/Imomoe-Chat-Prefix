# Imomoe-Chat-Prefix

Standalone shared chat-prefix Plugify plugin (`imomoe_chat_prefix`) supplying formatted or randomized chat prefixes to companion plugins without intercepting chat messages. Read [README](README.md) before changing config or exported API behavior.

## Workspace & hygiene

- Standalone clones default to `<repo>/.work/`. When `IMOMOE_WORK_ROOT` is set, place shared caches, task worktrees (`$IMOMOE_WORK_ROOT/worktrees/Imomoe-Chat-Prefix-<task>`), and disposable probes under it instead of creating repo-local `.work/` or committing ad-hoc scripts (`smoke*.py`, `drive-*.ps1`, `*_minimal.*`).
- Keep shipped defaults and `configs/imomoe_chat_prefix/prefix.example.toml` community-agnostic.

## Engineering & verification

- Inspect sibling call paths before patching; fix root causes rather than adding symptom workarounds. Apply one-in, one-out closeout to superseded tests, fixtures, and docs in the same diff.
- Verify changes with `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`.
