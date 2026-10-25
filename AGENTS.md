# AGENTS.md — Mandatory Operating Guidelines for AI Agents

> You are working on **SonyBridge** (https://github.com/DooperDrew/sonybridge).
> These rules are binding for every agent and every conversation in this repository.

## 1. Mission
SonyBridge is a lightweight, responsive Windows 10/11 desktop companion and system tray application for Sony Bluetooth headphones (specifically the WH-1000XM3, WH-1000XM4, WH-1000XM5, and WF series). It solves Windows Bluetooth connection headaches and brings the full functionality of the mobile "Sony | Headphones Connect" app to the PC desktop: real-time battery monitoring, Active Noise Canceling (ANC) / Ambient Sound control, 5-band hardware Equalizer & Clear Bass tuning, quick 1-click connect/reconnect, and smart Windows audio endpoint routing.

## 2. Mandatory reading order (before any implementation)
1. `docs/01-vision.md`: problem, principles, non-goals
2. `docs/02-requirements.md`: requirement IDs (F…, NFR-…) and budgets
3. `docs/03-architecture.md`: components, boundaries, key designs
4. `docs/04-ux.md`: UI/UX spec, system tray flows, and control schema
5. `docs/05-roadmap.md`: **the single source of truth for tasks**
6. `docs/<spike>-findings.md`: verified ground truth from M0 spikes. Don't contradict or guess beyond it.
7. `docs/decisions/`: ADRs. Changing a decision requires a new ADR.
8. `docs/lessons-learned.md` and `CHANGELOG.md`: history and pitfalls to avoid

## 3. Continuous Learning & Self-Correction Rule
Whenever the user corrects, redirects, or clarifies a requirement:
- **Do not silently apply the change.**
- You MUST explicitly formulate the takeaway as a rule and propose writing it into `docs/lessons-learned.md` (or `AGENTS.md` if binding).
- This ensures that lessons learned in one chat session immediately persist for all subsequent sessions.

## 4. Roles (multi-agent)
- **Lead agent (sole committer):** the only one allowed to `git commit`, `git push`, tag, release, or close issues. Plans parallel work, reviews worker output, runs the verification gate, and syncs docs.
- **Worker agents (`project-worker`):** work in isolated branch workspaces on **disjoint files**. **Forbidden:** `git commit`, `git push`, `git checkout main`, closing issues. They deliver results via the completion report in §8.
- Worker role names follow `<Tier>-<Domain>-<Milestone>` (e.g. `T1-Core-M2`, `T2-UI-M4`).

## 5. Complexity tiers & model selection
| Tier | Scope | Model |
|---|---|---|
| T1 Lean | pure logic, packet framing, parser, schemas, unit tests, i18n | `flash_lite` / `flash` |
| T2 Moderate | Tauri UI, system tray integration, settings storage, IPC commands | `flash` / `pro` |
| T3 Critical | WinRT Bluetooth RFCOMM/SPP transport, Windows Core Audio COM routing, MDR protocol state machine | `pro` / `inherit` |

Fallback order: `inherit` → `pro` → `flash` → `flash_lite`. Every fallback **must** be logged in `docs/model-fallback-log.md`, and an audit task for it goes into the hardening milestone.

## 6. Task lifecycle
- **Before:** find the task in `docs/05-roadmap.md`, restate the task + tier + target files + acceptance criteria, and check that `git status` is clean. Ensure toolchain PATH is initialized (PowerShell: `$env:USERPROFILE\.cargo\bin`).
- **During:** respect the hard rules (§7). Unverified assumptions are marked 🔬 and probed, not guessed. Decouple headless tests from physical QA (keep manual QA as Pending).
- **Gate (must pass before finishing):**
```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
```
- **After (lead only):** tick the roadmap box (`(YYYY-MM-DD, <commit>)`), add a CHANGELOG entry, update docs/ADRs if the design changed, add any lesson to `docs/lessons-learned.md`, commit (`M<x>.<y>: <summary>`), push, and update the issue.

## 7. Hard rules
- **Claims follow evidence:** Never claim a test, benchmark, or QA item passed without having run it. Hardware tests on physical headphones (WH-1000XM3) remain `Pending` until confirmed with empirical telemetry or user validation.
- **Pure Core Isolation:** All Sony MDR protocol packet framing, serialization, deserialization, and state transitions live in `crates/sonybridge-core` with `#![forbid(unsafe_code)]` and zero OS or UI dependencies.
- **Resource & Memory Budget:** Idle RAM footprint must remain ≤ 35 MB; idle CPU must be 0% (event-driven Bluetooth I/O, no polling loops).
- **Audio Integrity First:** Never mute, detach, or switch Windows audio endpoints without graceful fallback and user consent.
- **Environment resiliency:** Prepend `$env:USERPROFILE\.cargo\bin` in PowerShell commands to guard against PATH omissions in subshells.
- **Windows File Hygiene:** Files must be encoded in UTF-8 without BOM. When creating files via PowerShell, always use `-Encoding utf8`. Never commit build artifacts (`target/`, `dist/`).
- **Single Instance & Global OS Objects:** Windows only permits one process to manage the tray icon or single-instance named pipe mutex. Tests touching these must use unique identifiers.

## 8. Worker completion report (required format)
```markdown
### WORKER COMPLETION REPORT: <Role>
- Task: <ID + description>
- Model used: <model> (fallback? yes/no + reason)
- Files created/modified: <list>
- Verification: <each gate command: PASS/FAIL, test counts>
- Decisions & rationale: <numbered>
- Open issues / risks / follow-ups: <list>
```

## 9. Escalation
If reality contradicts the docs (tests, Bluetooth protocol behavior, Windows WinRT RFCOMM bugs): **stop**, report the evidence to the user, propose a probe, and update the docs only after the user agrees.

## 10. Build & run
```powershell
# Development run (Tauri GUI)
cargo tauri dev

# Run headless diagnostic probe (M0)
cargo run -p sonybridge-cli -- probe

# Workspace test suite
cargo test --workspace

# Production release build
cargo tauri build
```
