$ErrorActionPreference = "Stop"

$ProjectRoot = Resolve-Path "$PSScriptRoot\.."
Set-Location $ProjectRoot

$env:CARGO_TARGET_DIR = Join-Path $env:LOCALAPPDATA "Athena\rathenafr-discord-bot\target"
$Dist = Join-Path $ProjectRoot "dist"
New-Item -ItemType Directory -Force -Path $Dist | Out-Null

cargo build --release

$Binary = Join-Path $env:CARGO_TARGET_DIR "release\rathenafr-discord-bot.exe"
Copy-Item $Binary (Join-Path $Dist "rathenafr-discord-bot.exe") -Force

Write-Host "Binaire de release copié dans $Dist"
