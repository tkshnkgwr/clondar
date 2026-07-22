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

# 文字化け回避のための日本語文字コード動的生成
$versionLabel  = [char]0x30d0 + [char]0x30fc + [char]0x30b8 + [char]0x30e7 + [char]0x30f3          # "バージョン"
$internalLabel = [char]0x5185 + [char]0x90e8 + $versionLabel                                     # "内部バージョン"
$suitableLabel = [char]0x9069 + [char]0x5408 + $versionLabel                                     # "適合バージョン"

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

# 3. docs/ja/SPECIFICATION.md & docs/en/SPECIFICATION.md の更新
$specPaths = @("docs/ja/SPECIFICATION.md", "docs/en/SPECIFICATION.md")
foreach ($specPath in $specPaths) {
    if (Test-Path $specPath) {
        Write-Host "Updating $specPath..."
        $lines = [System.IO.File]::ReadAllLines($specPath, $utf8)
        $newLines = [System.Collections.Generic.List[string]]::new()
        
        $hasInternal = $false
        for ($i = 0; $i -lt $lines.Length; $i++) {
            if ($lines[$i] -match "\*\*$internalLabel\*\*" -or $lines[$i] -match "\*\*Internal Version\*\*") {
                $hasInternal = $true
                break
            }
        }
        
        for ($i = 0; $i -lt $lines.Length; $i++) {
            $line = $lines[$i]
            if ($line -match "\*\*$versionLabel\*\*") {
                $line = "**$versionLabel**: $NewVersion"
                $newLines.Add($line)
                if (-not $hasInternal) {
                    $newLines.Add("**$internalLabel**: $InternalVersion")
                }
            } elseif ($line -match "\*\*Version\*\*") {
                $line = "**Version**: $NewVersion"
                $newLines.Add($line)
                if (-not $hasInternal) {
                    $newLines.Add("**Internal Version**: $InternalVersion")
                }
            } elseif ($line -match "\*\*$internalLabel\*\*") {
                $line = "**$internalLabel**: $InternalVersion"
                $newLines.Add($line)
            } elseif ($line -match "\*\*Internal Version\*\*") {
                $line = "**Internal Version**: $InternalVersion"
                $newLines.Add($line)
            } else {
                $newLines.Add($line)
            }
        }
        [System.IO.File]::WriteAllLines($specPath, $newLines.ToArray(), $utf8)
    } else {
        Write-Warning "$specPath not found."
    }
}

# 4. docs/ja/TEST_REPORT.md & docs/en/TEST_REPORT.md の更新
$testPaths = @("docs/ja/TEST_REPORT.md", "docs/en/TEST_REPORT.md")
foreach ($testPath in $testPaths) {
    if (Test-Path $testPath) {
        Write-Host "Updating $testPath..."
        $lines = [System.IO.File]::ReadAllLines($testPath, $utf8)
        for ($i = 0; $i -lt $lines.Length; $i++) {
            if ($lines[$i] -match "\*\*$suitableLabel\*\*") {
                $lines[$i] = "**$suitableLabel**: Widget v$NewVersion (DPI-Aware Physical Coordination Model with Tauri v2 compatibility)"
                break
            } elseif ($lines[$i] -match "\*\*Suitable Version\*\*") {
                $lines[$i] = "**Suitable Version**: Widget v$NewVersion (DPI-Aware Physical Coordination Model with Tauri v2 compatibility)"
                break
            }
        }
        [System.IO.File]::WriteAllLines($testPath, $lines, $utf8)
    } else {
        Write-Warning "$testPath not found."
    }
}

Write-Host "Version bump completed successfully!"