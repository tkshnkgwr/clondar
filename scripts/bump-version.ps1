param (
    [Parameter(Mandatory=$true)]
    [string]$NewVersion
)

# セマンティックバージョン (X.Y.Z) のバリデーション
if ($NewVersion -notmatch '^\d+\.\d+\.\d+$') {
    Write-Error "Error: Version must be in semantic versioning format (e.g. 1.2.3)"
    exit 1
}

$InternalVersion = "$NewVersion.0"

Write-Host "Bumping version to $NewVersion (Internal: $InternalVersion)..."
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

# 3. docs/SPECIFICATION.md
$specPath = "docs/SPECIFICATION.md"
if (Test-Path $specPath) {
    Write-Host "Updating $specPath..."
    $lines = [System.IO.File]::ReadAllLines($specPath, $utf8)
    $newLines = [System.Collections.Generic.List[string]]::new()
    
    $hasInternal = $false
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "\*\*内部バージョン\*\*") {
            $hasInternal = $true
            break
        }
    }
    
    for ($i = 0; $i -lt $lines.Length; $i++) {
        $line = $lines[$i]
        if ($line -match "\*\*バージョン\*\*") {
            $line = "**バージョン**: $NewVersion"
            $newLines.Add($line)
            if (-not $hasInternal) {
                $newLines.Add("**内部バージョン**: $InternalVersion")
            }
        } elseif ($line -match "\*\*内部バージョン\*\*") {
            $line = "**内部バージョン**: $InternalVersion"
            $newLines.Add($line)
        } else {
            $newLines.Add($line)
        }
    }
    [System.IO.File]::WriteAllLines($specPath, $newLines.ToArray(), $utf8)
} else {
    Write-Warning "$specPath not found."
}

# 4. docs/TEST_REPORT.md
$testPath = "docs/TEST_REPORT.md"
if (Test-Path $testPath) {
    Write-Host "Updating $testPath..."
    $lines = [System.IO.File]::ReadAllLines($testPath, $utf8)
    for ($i = 0; $i -lt $lines.Length; $i++) {
        if ($lines[$i] -match "\*\*適合バージョン\*\*") {
            $lines[$i] = "**適合バージョン**: Widget v$NewVersion (DPI-Aware Physical Coordination Model with Tauri v2 compatibility)"
            break
        }
    }
    [System.IO.File]::WriteAllLines($testPath, $lines, $utf8)
} else {
    Write-Warning "$testPath not found."
}

# 5. ui/package.json
$packageJsonPath = "ui/package.json"
if (Test-Path $packageJsonPath) {
    Write-Host "Updating $packageJsonPath..."
    $content = [System.IO.File]::ReadAllText($packageJsonPath, $utf8)
    $content = $content -replace '"version"\s*:\s*"[^"]+"', "`"version`": `"$NewVersion`""
    [System.IO.File]::WriteAllText($packageJsonPath, $content, $utf8)
} else {
    Write-Warning "$packageJsonPath not found."
}

# 6. ui/src/App.jsx
$appJsxPath = "ui/src/App.jsx"
if (Test-Path $appJsxPath) {
    Write-Host "Updating $appJsxPath..."
    $content = [System.IO.File]::ReadAllText($appJsxPath, $utf8)
    $content = $content -replace "setVersion\('[^']+'\)", "setVersion('$NewVersion')"
    [System.IO.File]::WriteAllText($appJsxPath, $content, $utf8)
} else {
    Write-Warning "$appJsxPath not found."
}

# 7. ui/src/utils/tauri.js
$tauriJsPath = "ui/src/utils/tauri.js"
if (Test-Path $tauriJsPath) {
    Write-Host "Updating $tauriJsPath..."
    $content = [System.IO.File]::ReadAllText($tauriJsPath, $utf8)
    $content = $content -replace "return '[^']+'\s*;\s*//\s*fallback", "return '$NewVersion'; // fallback"
    [System.IO.File]::WriteAllText($tauriJsPath, $content, $utf8)
} else {
    Write-Warning "$tauriJsPath not found."
}

Write-Host "Version bump completed successfully!"

