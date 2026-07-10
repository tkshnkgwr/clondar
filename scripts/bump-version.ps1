param (
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# セマンティックバージョン (X.Y.Z) のバリデーション
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Error: Version must be in semantic versioning format (e.g. 1.2.3)"
    exit 1
}

Write-Host "Bumping version to $NewVersion..."
$utf8 = New-Object System.Text.UTF8Encoding($false)

# 1. src-tauri/Cargo.toml
$cargoPath = "src-tauri/Cargo.toml"
if (Test-Path $cargoPath) {
    Write-Host "Updating $cargoPath..."
    $lines = [System.IO.File]::ReadAllLines($cargoPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match '^version\s*=\s*"[^"]+"') {
            $lines[$i] = "version = `"$NewVersion`""
            break
        }
    }
    [System.IO.File]::WriteAllLines($cargoPath, $lines, $utf8)
} else {
    Write-Warning "$cargoPath not found."
}

# 2. src-tauri/tauri.conf.json
$tauriPath = "src-tauri/tauri.conf.json"
if (Test-Path $tauriPath) {
    Write-Host "Updating $tauriPath..."
    $content = [System.IO.File]::ReadAllText($tauriPath, $utf8)
    $content = $content -replace '"version"\s*:\s*"[^"]+"', "`"version`": `"$NewVersion`""
    [System.IO.File]::WriteAllText($tauriPath, $content, $utf8)
} else {
    Write-Warning "$tauriPath not found."
}

Write-Host "Version bump completed successfully!"
