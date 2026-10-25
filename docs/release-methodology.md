# Release & Versioning

## SemVer
- **MAJOR:** breaking changes (protocol overhauls, major architectural shifts) or graduating to 1.0.0.
- **MINOR:** backward-compatible features (e.g. support for new Sony headphone models, new audio presets, new tray widgets).
- **PATCH:** bug fixes, translation updates, protocol timing tuning.

## Single source of truth for the version
The version lives in the root `Cargo.toml` under `[workspace.package] version`. All member crates and `tauri.conf.json` reference this workspace version. Build scripts read it from there and never hardcode it.

## Release procedure (lead agent only, with user approval)
1. Bump the version in `Cargo.toml` + write the `CHANGELOG.md` entry.
2. Run the full verification gate:
   ```powershell
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cargo build --workspace --release
   ```
3. Build the release installers with Tauri:
   ```powershell
   cargo tauri build
   ```
4. `git commit -m "release: vX.Y.Z"` → `git tag -a vX.Y.Z -m "Release vX.Y.Z"` → `git push origin main --tags`.
5. Create GitHub Release via `gh release create vX.Y.Z <binaries> --notes-file <release-notes>`. Artifacts go to GitHub Releases, **never into git**.

## Pre-release checklist
- [ ] Clean working tree (`git status`).
- [ ] NFR budgets measured and confirmed for this version (see `docs/06-qa-and-validation.md`).
- [ ] Real-environment QA status listed in the release notes (Pass / Pending).
- [ ] No hardcoded UI strings; i18n translations complete.
- [ ] Windows installer and portable `.exe` launch smoothly on a clean Windows machine without crashing.

## Hotfix
Ship a patch version through the same pipeline and flag any revoked release on GitHub.
