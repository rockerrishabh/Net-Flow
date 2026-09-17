# Net Flow - Local Development MSIX Setup & Packaging Script
# Prerequisites: Windows 11, PowerShell, Windows SDK (makeappx.exe, signtool.exe), Rust toolchain

param(
    [string]$WasdkVersion = "1.7.250310001",
    [string]$CertSubject = "CN=NetFlow-Dev-Test",
    [string]$PfxPath = (Join-Path $env:TEMP "NetFlow_Dev_Signing.pfx"),
    [System.Security.SecureString]$Password = $null,
    [switch]$SkipCert,
    [switch]$SkipBuild,
    [switch]$SkipPackage,
    [switch]$SkipInstall
)

if ($null -eq $Password) {
    $rawPass = if ($env:NETFLOW_CERT_PASSWORD) { $env:NETFLOW_CERT_PASSWORD } else { [System.Guid]::NewGuid().ToString("N") }
    $Password = ConvertTo-SecureString $rawPass -AsPlainText -Force
}

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$RootDir = Split-Path -Parent $ScriptDir
$WinmdDir = Join-Path $RootDir "widget\winmd"
$TargetDir = Join-Path $RootDir "target"
$LayoutDir = Join-Path $TargetDir "msix"
$MsixPath = Join-Path $TargetDir "NetFlow.msix"

# Read manifest to extract Publisher if not explicitly specified
$manifestSrc = Join-Path $RootDir "widget\Package.appxmanifest"
if (Test-Path $manifestSrc) {
    $manifestRaw = Get-Content $manifestSrc -Raw
    if ($manifestRaw -match 'Publisher="([^"]+)"' -and (-not $PSBoundParameters.ContainsKey('CertSubject'))) {
        $foundPub = $matches[1]
        if ($foundPub -ne 'CN=REPLACE-WITH-PUBLISHER-ID') {
            $CertSubject = $foundPub
        }
    }
}

# 1. Certificate Creation / Validation for Local Sideloading
if (-not $SkipCert) {
    $needNewCert = $true
    if (Test-Path $PfxPath) {
        try {
            $existingCert = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($PfxPath, $Password)
            if ($existingCert.Subject -eq $CertSubject) {
                $cert = $existingCert
                $needNewCert = $false
                Write-Host "Certificate with subject '$CertSubject' found at $PfxPath." -ForegroundColor Green
            } else {
                Write-Host "Existing certificate subject ('$($existingCert.Subject)') does not match target '$CertSubject'. Regenerating..." -ForegroundColor Yellow
                Remove-Item $PfxPath -Force -ErrorAction SilentlyContinue
            }
        } catch {
            Remove-Item $PfxPath -Force -ErrorAction SilentlyContinue
        }
    }

    if ($needNewCert) {
        $existingMyCert = Get-ChildItem Cert:\CurrentUser\My -ErrorAction SilentlyContinue | Where-Object { $_.Subject -eq $CertSubject -and $_.HasPrivateKey } | Select-Object -First 1
        if ($existingMyCert) {
            $cert = $existingMyCert
            Write-Host "Found existing matching certificate in Cert:\CurrentUser\My: $CertSubject" -ForegroundColor Green
        } else {
            Write-Host "Creating self-signed developer certificate: $CertSubject..." -ForegroundColor Cyan
            $cert = New-SelfSignedCertificate `
                -Type Custom `
                -Subject $CertSubject `
                -KeyUsage DigitalSignature `
                -FriendlyName "Net Flow Local Dev Signing" `
                -CertStoreLocation "Cert:\CurrentUser\My" `
                -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")
        }

        $certDir = Split-Path -Parent $PfxPath
        if (-not (Test-Path $certDir)) {
            New-Item -ItemType Directory -Force -Path $certDir | Out-Null
        }

        Write-Host "Exporting PFX to $PfxPath..." -ForegroundColor Cyan
        Export-PfxCertificate -Cert $cert -FilePath $PfxPath -Password $Password | Out-Null
    }

    # Ensure certificate is trusted if local installation is requested
    if (-not $SkipInstall) {
        $inRoot = Get-ChildItem Cert:\LocalMachine\Root -ErrorAction SilentlyContinue | Where-Object { $_.Thumbprint -eq $cert.Thumbprint }
        $inTrustedPeople = Get-ChildItem Cert:\CurrentUser\TrustedPeople -ErrorAction SilentlyContinue | Where-Object { $_.Thumbprint -eq $cert.Thumbprint }

        if (-not $inTrustedPeople) {
            Write-Host "Adding certificate to Cert:\CurrentUser\TrustedPeople..." -ForegroundColor Cyan
            Import-PfxCertificate -CertStoreLocation "Cert:\CurrentUser\TrustedPeople" -FilePath $PfxPath -Password $Password | Out-Null
        }

        if (-not $inRoot) {
            Write-Host "Installing certificate to Cert:\LocalMachine\Root for local sideloading..." -ForegroundColor Yellow
            $cerPath = Join-Path $env:TEMP "netflow_dev_test.cer"
            [System.IO.File]::WriteAllBytes($cerPath, $cert.Export([System.Security.Cryptography.X509Certificates.X509ContentType]::Cert))
            try {
                Start-Process powershell -Verb RunAs -Wait -ArgumentList "-NoProfile -Command `"Import-Certificate -CertStoreLocation 'Cert:\LocalMachine\Root' -FilePath '$cerPath'`""
                Write-Host "Certificate installed to LocalMachine\Root." -ForegroundColor Green
            }
            catch {
                Write-Warning "Could not automatically elevate to install cert to LocalMachine\Root. If Add-AppxPackage fails, import '$cerPath' into Trusted Root Certification Authorities."
            }
            finally {
                Remove-Item $cerPath -Force -ErrorAction SilentlyContinue
            }
        }
    }
}

# 2. Download Windows App SDK winmd (if not already present)
if (-not (Test-Path (Join-Path $WinmdDir "Microsoft.Windows.Widgets.winmd"))) {
    Write-Host "Downloading Microsoft.WindowsAppSDK $WasdkVersion from NuGet..." -ForegroundColor Cyan
    $tempNupkg = Join-Path $env:TEMP "wasdk_$WasdkVersion.zip"
    $nupkgUrl = "https://api.nuget.org/v3-flatcontainer/microsoft.windowsappsdk/$WasdkVersion/microsoft.windowsappsdk.$WasdkVersion.nupkg"
    curl.exe -s -L $nupkgUrl -o $tempNupkg

    $tempExtract = Join-Path $env:TEMP "wasdk_extracted_$WasdkVersion"
    if (Test-Path $tempExtract) { Remove-Item $tempExtract -Recurse -Force }
    New-Item -ItemType Directory -Force -Path $tempExtract | Out-Null
    tar -xf $tempNupkg -C $tempExtract

    $foundWinmd = Get-ChildItem $tempExtract -Recurse -Filter "Microsoft.Windows.Widgets.winmd" | Select-Object -First 1
    if (-not $foundWinmd) {
        throw "Could not find Microsoft.Windows.Widgets.winmd in package!"
    }
    if (-not (Test-Path $WinmdDir)) {
        New-Item -ItemType Directory -Force -Path $WinmdDir | Out-Null
    }
    Copy-Item $foundWinmd.FullName $WinmdDir -Force
    Write-Host "Extracted winmd to $WinmdDir" -ForegroundColor Green

    Remove-Item $tempNupkg -Force -ErrorAction SilentlyContinue
    Remove-Item $tempExtract -Recurse -Force -ErrorAction SilentlyContinue
}
else {
    Write-Host "Microsoft.Windows.Widgets.winmd already present." -ForegroundColor Green
}

# 3. Build Rust Workspace
if (-not $SkipBuild) {
    Write-Host "Building release binary..." -ForegroundColor Cyan
    cargo build --release --manifest-path (Join-Path $RootDir "Cargo.toml")
    if ($LASTEXITCODE -ne 0) { throw "Cargo build failed!" }
}

# 4. Find Windows SDK Tools
$sdkVer = if ($env:NETFLOW_SDK_VERSION) { $env:NETFLOW_SDK_VERSION } elseif ($env:WindowsSDKVersion) { $env:WindowsSDKVersion } else { "10.0.26100.0" }
$sdkVer = $sdkVer.Trim('\', '/')
$sdkBin = "C:\Program Files (x86)\Windows Kits\10\bin\$sdkVer\x64"
$makeappx = Join-Path $sdkBin "makeappx.exe"
$signtool = Join-Path $sdkBin "signtool.exe"

if (-not (Test-Path $makeappx)) {
    $makeappx = (Get-Command makeappx.exe -ErrorAction SilentlyContinue).Source
}
if (-not (Test-Path $signtool)) {
    $signtool = (Get-Command signtool.exe -ErrorAction SilentlyContinue).Source
}

# 5. Prepare MSIX Layout
if (-not $SkipPackage) {
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 200

    Write-Host "Staging MSIX layout..." -ForegroundColor Cyan
    if (Test-Path $LayoutDir) { Remove-Item $LayoutDir -Recurse -Force }
    New-Item -ItemType Directory -Force -Path $LayoutDir | Out-Null

    $exeSrc = Join-Path $RootDir "target\release\net-flow.exe"
    if (-not (Test-Path $exeSrc)) { throw "net-flow.exe not found at $exeSrc" }
    Copy-Item $exeSrc $LayoutDir -Force

    # Read manifest and sync version from Cargo.toml
    $cargoToml = Get-Content (Join-Path $RootDir "Cargo.toml") -Raw
    $cargoVer = "0.1.0.0"
    if ($cargoToml -match 'version\s*=\s*"([^"]+)"') {
        $semVer = $matches[1]
        $parts = $semVer.Split('.')
        if ($parts.Count -eq 3) {
            $cargoVer = "$semVer.0"
        }
        elseif ($parts.Count -eq 4) {
            $cargoVer = $semVer
        }
    }
    Write-Host "Syncing manifest version: $cargoVer" -ForegroundColor Cyan

    $manifestSrc = Join-Path $RootDir "widget\Package.appxmanifest"
    $manifestContent = Get-Content $manifestSrc -Raw
    $manifestContent = $manifestContent -creplace '(?<=<Identity\b[^>]*?\sVersion=")[0-9\.]+', $cargoVer

    # Only synchronize Publisher to local dev cert if manifest has placeholder or if requested
    if (-not $SkipCert -and ($manifestContent -match 'Publisher="CN=REPLACE-WITH-PUBLISHER-ID"')) {
        $manifestContent = $manifestContent -creplace '(?<=<Identity\b[^>]*?\sPublisher=")[^"]+', $CertSubject
    }

    $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText((Join-Path $LayoutDir "AppxManifest.xml"), $manifestContent, $utf8NoBom)

    $assetsSrc = Join-Path $RootDir "widget\Assets"
    if (Test-Path $assetsSrc) {
        Copy-Item $assetsSrc (Join-Path $LayoutDir "Assets") -Recurse -Force
    }
    New-Item -ItemType Directory -Force -Path (Join-Path $LayoutDir "Public") | Out-Null

    # 6. Index resources with makepri
    Write-Host "Generating resources.pri..." -ForegroundColor Cyan
    $makepri = Join-Path $sdkBin "makepri.exe"
    if (-not (Test-Path $makepri)) {
        $makepri = (Get-Command makepri.exe -ErrorAction SilentlyContinue).Source
    }
    if ($makepri) {
        $priconfig = Join-Path $LayoutDir "priconfig.xml"
        & $makepri createconfig /cf $priconfig /dq "en-US" /pv "10.0.0" /o | Out-Null
        & $makepri new /pr $LayoutDir /cf $priconfig /of (Join-Path $LayoutDir "resources.pri") /o | Out-Null
        Remove-Item $priconfig -Force -ErrorAction SilentlyContinue
    }

    # 7. Pack MSIX
    Write-Host "Packing MSIX..." -ForegroundColor Cyan
    if (Test-Path $MsixPath) { Remove-Item $MsixPath -Force -ErrorAction SilentlyContinue }
    & $makeappx pack /d $LayoutDir /p $MsixPath /o
    if ($LASTEXITCODE -ne 0) { throw "makeappx failed!" }

    # 7. Sign MSIX
    if (-not $SkipCert) {
        Write-Host "Signing MSIX..." -ForegroundColor Cyan
        $bstr = [System.Runtime.InteropServices.Marshal]::SecureStringToBSTR($Password)
        $plainPwd = [System.Runtime.InteropServices.Marshal]::PtrToStringAuto($bstr)
        try {
            & $signtool sign /fd SHA256 /f $PfxPath /p $plainPwd $MsixPath
            if ($LASTEXITCODE -ne 0) { throw "signtool failed!" }
        }
        finally {
            [System.Runtime.InteropServices.Marshal]::ZeroFreeBSTR($bstr)
        }
        Write-Host "MSIX packaged and signed at $MsixPath" -ForegroundColor Green
    }

    # 8. Sideload Package
    if (-not $SkipInstall) {
        Write-Host "Installing package..." -ForegroundColor Cyan
        Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue | Remove-AppxPackage -ErrorAction SilentlyContinue
        Start-Sleep -Milliseconds 500
        Add-AppxPackage $MsixPath -ForceApplicationShutdown

        Stop-Process -Name "WidgetBoard", "WidgetService", "Widgets" -Force -ErrorAction SilentlyContinue
        Write-Host "Successfully installed NetFlow widget package and refreshed widget board!" -ForegroundColor Green
    }
}
