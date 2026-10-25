# Changelog & Development Log

Chronological record of milestones, decisions and discoveries. Newest first.

## 2026-10-25 — Project Definition & Spec Skeleton
- Bootstrapped project via `project-kickstart` skill.
- Generated core specification:
  - `docs/01-vision.md`: Defined core pain point, target users, guiding principles, and v1 non-goals.
  - `docs/02-requirements.md`: Specified functional requirements (F1–F8) and performance budgets (NFR-P1 to NFR-P5).
  - `docs/03-architecture.md`: Architecture specification with Tauri v2 + Rust Core, module boundaries, and Mermaid diagrams.
  - `docs/04-ux.md`: System tray layout, main dashboard UX flow, and hardware sync rules.
  - `docs/05-roadmap.md`: Sized, parallel-ready milestone roadmap from M0 spikes to M7 retrospective.
  - `docs/06-qa-and-validation.md`: Verification levels, NFR budget tracking, and real-environment hardware QA matrix.
  - `docs/decisions/0001-rust-tauri-stack.md`: ADR accepting Rust + Tauri v2 architecture.
  - `AGENTS.md`: Mandatory operating guidelines, verification gate, and complexity tiers.
- Open Spikes: M0.2 (WinRT/Winsock RFCOMM connectivity), M0.3 (Sony MDR protocol framing & handshake), M0.4 (Windows Core Audio endpoint routing).
