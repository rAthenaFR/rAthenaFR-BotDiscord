@echo off
setlocal
set CARGO_TARGET_DIR=%LOCALAPPDATA%\Athena\rathenafr-discord-bot\target
cargo build --release
endlocal
