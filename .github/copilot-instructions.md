# Instructions for AI coding assistants (Copilot, etc.)

Follow [AGENTS.md](../AGENTS.md) strictly. Essentials:
1. Read `docs/05-roadmap.md` and work on one task at a time.
2. Respect the hard rules and performance budgets in AGENTS.md §7.
3. Run the verification gate in AGENTS.md §6 before finishing any task.
4. Never present unverified hardware or OS behavior as fact. Mark it 🔬 and probe it.
5. All Sony MDR protocol logic must stay in `crates/sonybridge-core` with `#![forbid(unsafe_code)]` and zero OS dependencies.
