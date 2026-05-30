$ErrorActionPreference = "Stop"

$ProjectRoot = Resolve-Path "$PSScriptRoot\.."
$ExternalTarget = Join-Path $env:LOCALAPPDATA "Athena\rathenafr-discord-bot\target"

$Paths = @(
    (Join-Path $ProjectRoot "target"),
    (Join-Path $ProjectRoot "dist"),
    $ExternalTarget
)

foreach ($Path in $Paths) {
    if (Test-Path $Path) {
        Write-Host "Suppression de $Path"
        Remove-Item $Path -Recurse -Force
    }
}

Write-Host "Dossiers de build nettoyés."
