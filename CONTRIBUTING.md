# Contributing to SonyBridge

Contributions, bug reports, and protocol captures for additional Sony models (`WH-1000XM3`, `WH-1000XM4`, `WH-1000XM5`, `WF-1000XM4`, `WF-1000XM5`) are welcome!

## Development Prerequisites
- Windows 10 (1903+) or Windows 11
- Rust stable toolchain (`rustup`)
- Microsoft Edge WebView2 Runtime (pre-installed on Windows 10/11)

## Verification Gate
Before opening a pull request, ensure all four workspace checks pass with zero warnings:
```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
```

## Capturing Hardware Diagnostic Logs
If you are testing with physical Sony headphones, run the headless diagnostic probe and paste the JSON output into Issue `#10` or `#11`:
```powershell
.\sonybridge-cli.exe probe
```
