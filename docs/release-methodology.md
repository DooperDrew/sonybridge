# Release & Versioning

## SemVer
- **MAJOR:** breaking changes (protocol overhauls, major architectural shifts, incompatible config changes) or graduating to 1.0.0.
- **MINOR:** backward-compatible features (e.g. support for new Sony headphone models, new Sound Profiles, new tray tabs).
- **PATCH:** bug fixes, translation updates, protocol timing tuning.

## Single source of truth for the version
The version lives in the root `Cargo.toml` under `[workspace.package] version`. All member crates (`sonybridge-core`, `sonybridge-windows`, `sonybridge-cli`, `sonybridge-app`) and `crates/sonybridge-app/tauri.conf.json` stay synchronized with this workspace version.

## Release procedure (lead agent only)
1. Bump the version in `Cargo.toml` and `crates/sonybridge-app/tauri.conf.json` + write the `CHANGELOG.md` entry (+ update `README.md` download links).
2. Run the full verification gate:
   ```powershell
   $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   cargo build --workspace --release
   ```
3. Package the portable `.zip` release bundle and update Scoop / WinGet manifests:
   ```powershell
   .\scripts\package-release.ps1 -Version "X.Y.Z" -SkipBuild
   .\scripts\generate-manifests.ps1 -Version "X.Y.Z"
   ```
4. `git commit -m "release: vX.Y.Z"` → `git push origin main`.
5. Create the GitHub Release and attach the portable `.zip` archive via `gh release create vX.Y.Z "dist/SonyBridge-vX.Y.Z-windows-x64.zip" --title "..." --notes "..." --latest`. Build artifacts (`target/`, `dist/`) go to GitHub Releases, **never into git**.

## Pre-release checklist
- [ ] Clean working tree (`git status`).
- [ ] NFR budgets respected for this version (see `docs/06-qa-and-validation.md`).
- [ ] Real-environment hardware QA status listed in `docs/06-qa-and-validation.md` (`Pass` / `Pending`).
- [ ] No hardcoded UI strings; EN/IT i18n translations complete.
- [ ] Portable `.zip` archive verified to contain `sonybridge-app.exe`, `sonybridge-cli.exe`, `setup.cmd`, `setup-wizard.ps1`, `install.ps1`, `uninstall.ps1`, `locales/`, and docs.

## Hotfix
Ship a patch version through the same pipeline and flag any revoked release on GitHub.
