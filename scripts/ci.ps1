$ErrorActionPreference = "Stop"

$ProjectRoot = Resolve-Path "$PSScriptRoot\.."
Set-Location $ProjectRoot

$env:CARGO_TARGET_DIR = Join-Path $env:LOCALAPPDATA "Athena\rathenafr-discord-bot\target"

cargo fmt --all -- --check
cargo check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
