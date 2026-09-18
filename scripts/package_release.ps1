# Net Flow - Local Release Packaging and Verification Script
# Builds, stages, packages, signs, and computes checksums for all release artifacts defined in README.md.

param(
    [string]$Version = "",
    [switch]$SkipBuild,
    [switch]$SkipMsix,
    [switch]$SkipZip,
    [switch]$Clean
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir
$TargetDir = Join-Path $RootDir "target"

Write-Host "=================================================" -ForegroundColor Cyan
Write-Host " Net Flow - Release Packaging and Verification" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# 1. Resolve Version
if ([string]::IsNullOrWhiteSpace($Version)) {
    $cargoTomlPath = Join-Path $RootDir "Cargo.toml"
    if (Test-Path $cargoTomlPath) {
        $cargoLines = Get-Content $cargoTomlPath
        foreach ($line in $cargoLines) {
            if ($line -match '^\s*version\s*=\s*"([^"]+)"') {
                $Version = $matches[1]
                break
            }
        }
    }
}

if ([string]::IsNullOrWhiteSpace($Version)) {
    $Version = "0.1.0"
}
Write-Host "Target Version: $Version" -ForegroundColor Green

if ($Clean) {
    Write-Host "Cleaning release staging directories..." -ForegroundColor Cyan
    Remove-Item (Join-Path $TargetDir "msix") -Recurse -Force -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $TargetDir "portable") -Recurse -Force -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $TargetDir "net-flow-windows-x64.zip") -Force -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $TargetDir "SHA256SUMS.txt") -Force -ErrorAction SilentlyContinue
    Remove-Item (Join-Path $TargetDir "RELEASE_NOTES.md") -Force -ErrorAction SilentlyContinue
}

# 2. Build Release Executable
$releaseExe = Join-Path $TargetDir "release\net-flow.exe"
if (-not $SkipBuild) {
    Write-Host "`n--- [Step 1/5] Building Release Binary (Cargo LTO) ---" -ForegroundColor Cyan
    Push-Location $RootDir
    try {
        & cargo build --release --workspace
        if ($LASTEXITCODE -ne 0) { throw "Cargo build failed!" }
    } finally {
        Pop-Location
    }
} else {
    Write-Host "`n--- [Step 1/5] Skipping Cargo Build (-SkipBuild specified) ---" -ForegroundColor Yellow
}

if (-not (Test-Path $releaseExe)) {
    throw "Release binary not found at $releaseExe!"
}
$exeSizeMb = [math]::Round(((Get-Item $releaseExe).Length / 1MB), 2)
Write-Host "Release binary verified: $releaseExe ($exeSizeMb MB)" -ForegroundColor Green

# 3. Package and Sign MSIX
$msixPath = Join-Path $TargetDir "NetFlow.msix"
$cerPath = Join-Path $TargetDir "NetFlow_Sideload_Cert.cer"

if (-not $SkipMsix) {
    Write-Host "`n--- [Step 2/5] Packaging and Signing MSIX Package ---" -ForegroundColor Cyan
    $sdkBin = "C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64"
    $makeappx = Join-Path $sdkBin "makeappx.exe"
    if (-not (Test-Path $makeappx)) {
        $makeappx = (Get-Command makeappx.exe -ErrorAction SilentlyContinue).Source
    }
    $signtool = Join-Path $sdkBin "signtool.exe"
    if (-not (Test-Path $signtool)) {
        $signtool = (Get-Command signtool.exe -ErrorAction SilentlyContinue).Source
    }

    if ($makeappx -and $signtool) {
        $layoutDir = Join-Path $TargetDir "msix"
        if (Test-Path $layoutDir) { Remove-Item $layoutDir -Recurse -Force }
        New-Item -ItemType Directory -Force -Path $layoutDir | Out-Null

        Copy-Item $releaseExe $layoutDir -Force

        # Prepare Manifest
        $manifestSrc = Join-Path $RootDir "widget\Package.appxmanifest"
        $manifestContent = Get-Content $manifestSrc -Raw
        $quadVer = if ($Version.Split('.').Count -eq 3) { "$Version.0" } else { $Version }
        $manifestContent = $manifestContent -creplace '(?<=<Identity\b[^>]*?\sVersion=")[0-9\.]+', $quadVer
        $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText((Join-Path $layoutDir "AppxManifest.xml"), $manifestContent, $utf8NoBom)

        # Assets
        $assetsSrc = Join-Path $RootDir "widget\Assets"
        if (Test-Path $assetsSrc) {
            Copy-Item $assetsSrc (Join-Path $layoutDir "Assets") -Recurse -Force
        }
        New-Item -ItemType Directory -Force -Path (Join-Path $layoutDir "Public") | Out-Null

        # MakePRI
        $makepri = Join-Path $sdkBin "makepri.exe"
        if (-not (Test-Path $makepri)) {
            $makepri = (Get-Command makepri.exe -ErrorAction SilentlyContinue).Source
        }
        if ($makepri) {
            $priconfig = Join-Path $layoutDir "priconfig.xml"
            & $makepri createconfig /cf $priconfig /dq "en-US" /pv "10.0.0" /o | Out-Null
            & $makepri new /pr $layoutDir /cf $priconfig /of (Join-Path $layoutDir "resources.pri") /o | Out-Null
            Remove-Item $priconfig -Force -ErrorAction SilentlyContinue
        }

        # Pack MSIX
        if (Test-Path $msixPath) { Remove-Item $msixPath -Force -ErrorAction SilentlyContinue }
        & $makeappx pack /d $layoutDir /p $msixPath /o | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "makeappx pack failed!" }

        # Certificate and Signing (Subject MUST match manifest Publisher)
        $certSubject = "CN=Development"
        if ($manifestContent -match 'Publisher="([^"]+)"') {
            $certSubject = $matches[1]
        }
        $existingCert = Get-ChildItem Cert:\CurrentUser\My -ErrorAction SilentlyContinue | Where-Object { $_.Subject -eq $certSubject -and $_.HasPrivateKey } | Select-Object -First 1
        if (-not $existingCert) {
            Write-Host "Creating signing certificate matching manifest Publisher ($certSubject)..." -ForegroundColor Cyan
            $existingCert = New-SelfSignedCertificate `
                -Type Custom `
                -Subject $certSubject `
                -KeyUsage DigitalSignature `
                -FriendlyName "Net Flow Release Signing" `
                -CertStoreLocation "Cert:\CurrentUser\My" `
                -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")
        }

        # Export .cer
        [System.IO.File]::WriteAllBytes($cerPath, $existingCert.Export([System.Security.Cryptography.X509Certificates.X509ContentType]::Cert))

        # Sign MSIX
        $pfxTemp = Join-Path $env:TEMP "netflow_release_sign.pfx"
        $pfxPass = ConvertTo-SecureString "NetFlowPass123!" -AsPlainText -Force
        Export-PfxCertificate -Cert $existingCert -FilePath $pfxTemp -Password $pfxPass | Out-Null
        try {
            & $signtool sign /fd SHA256 /f $pfxTemp /p "NetFlowPass123!" $msixPath | Out-Null
            if ($LASTEXITCODE -ne 0) { throw "signtool failed to sign $msixPath" }
        } finally {
            Remove-Item $pfxTemp -Force -ErrorAction SilentlyContinue
        }

        Write-Host "Packaged and signed MSIX: $msixPath" -ForegroundColor Green
        Write-Host "Exported public sideload cert: $cerPath" -ForegroundColor Green
    } else {
        Write-Host "Windows SDK makeappx or signtool not found. Skipping MSIX build." -ForegroundColor Yellow
    }
} else {
    Write-Host "`n--- [Step 2/5] Skipping MSIX Packaging (-SkipMsix specified) ---" -ForegroundColor Yellow
}

# 4. Assemble Portable Archive (net-flow-windows-x64.zip)
$zipPath = Join-Path $TargetDir "net-flow-windows-x64.zip"
if (-not $SkipZip) {
    Write-Host "`n--- [Step 3/5] Assembling Portable Archive ($zipPath) ---" -ForegroundColor Cyan
    $portableStage = Join-Path $TargetDir "portable"
    if (Test-Path $portableStage) { Remove-Item $portableStage -Recurse -Force }
    New-Item -ItemType Directory -Force -Path $portableStage | Out-Null

    Copy-Item $releaseExe (Join-Path $portableStage "net-flow.exe") -Force
    Copy-Item (Join-Path $RootDir "widget\Assets") (Join-Path $portableStage "Assets") -Recurse -Force
    Copy-Item (Join-Path $RootDir "README.md") (Join-Path $portableStage "README.md") -Force
    Copy-Item (Join-Path $RootDir "LICENSE-MIT") (Join-Path $portableStage "LICENSE-MIT") -Force
    Copy-Item (Join-Path $RootDir "LICENSE-APACHE") (Join-Path $portableStage "LICENSE-APACHE") -Force

    $regScript = Join-Path $RootDir "scripts\register.ps1"
    if (Test-Path $regScript) {
        Copy-Item $regScript (Join-Path $portableStage "register.ps1") -Force
    }

    $manifestPath = Join-Path $layoutDir "AppxManifest.xml"
    if (Test-Path $manifestPath) {
        Copy-Item $manifestPath (Join-Path $portableStage "AppxManifest.xml") -Force
    }
    $priPath = Join-Path $layoutDir "resources.pri"
    if (Test-Path $priPath) {
        Copy-Item $priPath (Join-Path $portableStage "resources.pri") -Force
    }

    if (Test-Path $cerPath) {
        Copy-Item $cerPath (Join-Path $portableStage "NetFlow_Sideload_Cert.cer") -Force
    }

    if (Test-Path $zipPath) { Remove-Item $zipPath -Force }
    Compress-Archive -Path (Join-Path $portableStage "*") -DestinationPath $zipPath -Force
    Remove-Item $portableStage -Recurse -Force -ErrorAction SilentlyContinue

    Write-Host "Portable archive created successfully: $zipPath" -ForegroundColor Green
} else {
    Write-Host "`n--- [Step 3/5] Skipping Portable Zip (-SkipZip specified) ---" -ForegroundColor Yellow
}

# 5. Extract Release Notes from CHANGELOG.md
Write-Host "`n--- [Step 4/5] Extracting Release Notes from CHANGELOG.md ---" -ForegroundColor Cyan
$changelogPath = Join-Path $RootDir "CHANGELOG.md"
$releaseNotesPath = Join-Path $TargetDir "RELEASE_NOTES.md"

if (Test-Path $changelogPath) {
    $rawChangelog = Get-Content $changelogPath -Raw
    $escapedVer = [regex]::Escape($Version)
    $pattern = "(?s)## \[$escapedVer\][^\r\n]*\r?\n(.*?)(?=\r?\n## \[|\Z)"
    if ($rawChangelog -match $pattern) {
        $extractedNotes = $matches[1].Trim()
        $header = "# Net Flow v$Version`n`n"
        [System.IO.File]::WriteAllText($releaseNotesPath, $header + $extractedNotes + "`n", [System.Text.Encoding]::UTF8)
        Write-Host "Extracted version v$Version notes to $releaseNotesPath" -ForegroundColor Green
    } else {
        Write-Host "Version [$Version] section not explicitly matched in CHANGELOG.md. Using summary." -ForegroundColor Yellow
        $summaryNotes = "# Net Flow v$Version`n`nStandalone release $Version.`n"
        [System.IO.File]::WriteAllText($releaseNotesPath, $summaryNotes, [System.Text.Encoding]::UTF8)
    }
}

# 6. Generate Checksums (SHA256SUMS.txt)
Write-Host "`n--- [Step 5/5] Generating SHA256SUMS.txt Checksums ---" -ForegroundColor Cyan
$sumsPath = Join-Path $TargetDir "SHA256SUMS.txt"
$filesToCheck = @($msixPath, $zipPath, $cerPath)
$checksumLines = @()
$summaryRows = @()

foreach ($filePath in $filesToCheck) {
    if (Test-Path $filePath) {
        $hash = (Get-FileHash -Path $filePath -Algorithm SHA256).Hash.ToLowerInvariant()
        $fileName = Split-Path -Leaf $filePath
        $checksumLines += "$hash  $fileName"

        $size = [math]::Round(((Get-Item $filePath).Length / 1MB), 2)
        $summaryRows += [PSCustomObject]@{
            FileName = $fileName
            "Size (MB)" = $size
            SHA256 = $hash.Substring(0, 16) + "..."
        }
    }
}

[System.IO.File]::WriteAllLines($sumsPath, $checksumLines, [System.Text.Encoding]::UTF8)
Write-Host "Generated checksums file: $sumsPath" -ForegroundColor Green

# 7. Summary
Write-Host "`n=================================================" -ForegroundColor Cyan
Write-Host " Release Artifacts Prepared Successfully!" -ForegroundColor Green
Write-Host "=================================================" -ForegroundColor Cyan
$summaryRows | Format-Table -AutoSize
Write-Host "Release Notes Path: $releaseNotesPath" -ForegroundColor Gray
Write-Host "Checksums File    : $sumsPath" -ForegroundColor Gray
