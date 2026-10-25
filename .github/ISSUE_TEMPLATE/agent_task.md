---
name: "🤖 Agent Implementation Task"
about: Structured implementation task designed for GitHub Copilot Agent or autonomous worker subagents
title: "[Task]: "
labels: ["agent-task", "enhancement"]
assignees: []
---

## 🎯 Task Objective & Scope
<!-- A clear 1-2 sentence description of what needs to be implemented and why -->

## 📊 Complexity Tier & Target
- **Complexity Tier:** Tier 1 (Lean) | Tier 2 (Moderate) | Tier 3 (Complex / Critical)
- **Target Module / Crate:** `crates/sonybridge-core` | `crates/sonybridge-windows` | `crates/sonybridge-cli` | `crates/sonybridge-app`
- **Target Files (Explicit Ownership):**
  - `path/to/target/file.rs`
  - `path/to/test/file.rs`

## 🛡️ Architectural & Performance Invariants
<!-- Specify any critical constraints (check all that apply) -->
- [ ] **Pure Logic (`sonybridge-core`):** Core Sony MDR protocol logic isolated from OS/UI side effects with `#![forbid(unsafe_code)]`, 100% unit-tested.
- [ ] **Performance Budgets:** Comply with non-functional budgets defined in `docs/02-requirements.md` (Idle RAM <= 35 MB, 0% idle CPU, event-driven I/O).
- [ ] **Safety & Invariants:** Any `unsafe` COM/Win32 operations in `sonybridge-windows` must have documented preconditions, scoped lifetimes before `CoUninitialize()`, and proper `CoTaskMemFree` cleanup.
- [ ] **Localization:** User-facing strings must reside in `crates/sonybridge-core/src/i18n.rs` and `crates/sonybridge-app/locales/{en,it}.toml`.
- [ ] **Claims Follow Evidence:** Never claim a test, benchmark, or hardware QA item passed without empirical verification.

## 📝 Implementation Instructions
<!-- Step-by-step guidance for the coding agent -->
1. 
2. 
3. 

## ✅ Verifiable Acceptance Criteria
<!-- Checkboxes that can be verified mechanically -->
- [ ] Unit tests added covering all new logic, branches, and error cases.
- [ ] `cargo fmt --all -- --check` passes cleanly.
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` produces zero warnings or errors.
- [ ] `cargo test --workspace` passes 100%.
- [ ] `cargo build --workspace --release` succeeds.

## 🔄 Sole Committer Protocol Reminder
> **Note for AI Agents:** Deliver your changes via a **Draft Pull Request** on branch `task/<issue-id>-<description>`. Do not commit directly to `main` or attempt to close this issue. The Lead Agent (Antigravity) will review, run the integration verification gate, and merge.
