# Net Flow - Windows 11 Widget Installer Helper
# Automatically generates a local signing certificate, packages NetFlow.msix from layout,
# trusts the certificate in Local Computer -> Trusted People, and installs the package into Windows 11.

[CmdletBinding()]
param(
    [switch]$Install,
    [switch]$Uninstall,
    [switch]$RemoveCert,
    [switch]$Rebuild,
    [switch]$Status,
    [switch]$RestartWidgets,
    [string]$MsixPath = ""
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ExePath = Join-Path $ScriptDir "net-flow.exe"
$ManifestPath = Join-Path $ScriptDir "AppxManifest.xml"
$DefaultMsix = Join-Path $ScriptDir "NetFlow.msix"

function Show-Header {
    Write-Host "=================================================" -ForegroundColor Cyan
    Write-Host "  Net Flow - Windows 11 Widget Installer" -ForegroundColor Cyan
    Write-Host "=================================================" -ForegroundColor Cyan
}

function Find-SdkTools {
    $tools = @{
        MakeAppx = $null
        SignTool = $null
        MakePri  = $null
    }

    $sdkBase = "C:\Program Files (x86)\Windows Kits\10\bin"
    if (Test-Path $sdkBase) {
        $sdkDirs = Get-ChildItem $sdkBase -Directory -ErrorAction SilentlyContinue |
            Where-Object { $_.Name -match '^\d+\.\d+\.\d+\.\d+$' } |
            Sort-Object { [version]$_.Name } -Descending

        foreach ($sdkDir in $sdkDirs) {
            $x64Path = Join-Path $sdkDir.FullName "x64"
            if (Test-Path (Join-Path $x64Path "makeappx.exe")) {
                $tools.MakeAppx = Join-Path $x64Path "makeappx.exe"
                $tools.SignTool = Join-Path $x64Path "signtool.exe"
                $tools.MakePri  = Join-Path $x64Path "makepri.exe"
                break
            }
        }
    }

    if (-not $tools.MakeAppx) {
        $cmd = Get-Command makeappx.exe -ErrorAction SilentlyContinue
        if ($cmd) { $tools.MakeAppx = $cmd.Source }
    }
    if (-not $tools.SignTool) {
        $cmd = Get-Command signtool.exe -ErrorAction SilentlyContinue
        if ($cmd) { $tools.SignTool = $cmd.Source }
    }
    if (-not $tools.MakePri) {
        $cmd = Get-Command makepri.exe -ErrorAction SilentlyContinue
        if ($cmd) { $tools.MakePri = $cmd.Source }
    }

    return $tools
}

function Get-PackageManifestPath {
    $candidates = @(
        $ManifestPath,
        (Join-Path $ScriptDir "..\widget\Package.appxmanifest"),
        (Join-Path $ScriptDir "AppxManifest.xml")
    )
    foreach ($cand in $candidates) {
        if (Test-Path $cand) { return (Resolve-Path $cand).Path }
    }
    return $null
}

function Get-PackageIdentity {
    $manifest = Get-PackageManifestPath
    if ($manifest -and (Test-Path $manifest)) {
        $content = Get-Content $manifest -Raw
        if ($content -match '<Identity\b[^>]*?\sName="([^"]+)"') {
            return $matches[1]
        }
    }
    return "NetFlow.Widget"
}

function Get-PackagePublisher {
    $manifest = Get-PackageManifestPath
    if ($manifest -and (Test-Path $manifest)) {
        $content = Get-Content $manifest -Raw
        if ($content -match '<Identity\b[^>]*?\sPublisher="([^"]+)"') {
            return $matches[1]
        }
    }
    return "CN=Development"
}

function Find-ExistingBinary {
    $candidates = @(
        $ExePath,
        (Join-Path $ScriptDir "..\target\release\net-flow.exe"),
        (Join-Path $ScriptDir "..\target\x86_64-pc-windows-msvc\release\net-flow.exe"),
        (Join-Path $ScriptDir "..\target\aarch64-pc-windows-msvc\release\net-flow.exe")
    )
    foreach ($cand in $candidates) {
        if (Test-Path $cand) { return (Resolve-Path $cand).Path }
    }
    return $null
}

function Find-ExistingMsix {
    if ($MsixPath -and (Test-Path $MsixPath)) {
        return (Resolve-Path $MsixPath).Path
    }

    $candidates = @(
        $DefaultMsix,
        (Join-Path $ScriptDir "target\NetFlow.msix"),
        (Join-Path $ScriptDir "..\target\NetFlow.msix")
    )

    foreach ($cand in $candidates) {
        if (Test-Path $cand) {
            return (Resolve-Path $cand).Path
        }
    }

    $anyMsix = Get-ChildItem -Path $ScriptDir -Filter "*.msix" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($anyMsix) {
        return $anyMsix.FullName
    }

    return $null
}

function Install-SideloadCert {
    param([System.Security.Cryptography.X509Certificates.X509Certificate2]$Certificate)

    if (-not $Certificate) { return }
    $thumb = $Certificate.Thumbprint

    # Device trust for MSIX package sideloading must be in LocalMachine\TrustedPeople
    $inLocalTrusted = Get-ChildItem Cert:\LocalMachine\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Thumbprint -eq $thumb }

    if (-not $inLocalTrusted) {
        $tempCer = Join-Path $env:TEMP ("netflow_cert_" + $thumb + ".cer")
        [System.IO.File]::WriteAllBytes($tempCer, $Certificate.Export([System.Security.Cryptography.X509Certificates.X509ContentType]::Cert))

        # Try direct import into LocalMachine\TrustedPeople (succeeds if running elevated)
        try {
            Import-Certificate -CertStoreLocation "Cert:\LocalMachine\TrustedPeople" -FilePath $tempCer -ErrorAction Stop | Out-Null
            Write-Host "Package signing certificate installed to LocalMachine\TrustedPeople." -ForegroundColor Green
        } catch {
            Write-Host "Sideloading requires trusting the package certificate in LocalMachine\TrustedPeople." -ForegroundColor Yellow
            Write-Host "Prompting for administrator approval to trust certificate..." -ForegroundColor Cyan
            try {
                $argList = "-NoProfile -ExecutionPolicy Bypass -Command `"Import-Certificate -CertStoreLocation 'Cert:\LocalMachine\TrustedPeople' -FilePath '$tempCer' -ErrorAction Stop | Out-Null`""
                $proc = Start-Process powershell -Verb RunAs -Wait -PassThru -ArgumentList $argList
                if ($proc.ExitCode -eq 0) {
                    Write-Host "Certificate installed to LocalMachine\TrustedPeople successfully." -ForegroundColor Green
                } else {
                    Write-Warning "Elevation returned exit code $($proc.ExitCode)."
                }
            } catch {
                Write-Warning "Could not elevate to install certificate. If installation fails, right-click install.ps1 -> Run with PowerShell as Administrator."
            }
        } finally {
            Remove-Item $tempCer -Force -ErrorAction SilentlyContinue
        }
    } else {
        Write-Host "Package signing certificate is trusted in LocalMachine\TrustedPeople." -ForegroundColor Green
    }
}

function New-MsixPackage {
    $tools = Find-SdkTools
    if (-not $tools.MakeAppx -or -not $tools.SignTool) {
        $existing = Find-ExistingMsix
        if ($existing) {
            Write-Host "Windows SDK tools (makeappx/signtool) not found; falling back to existing MSIX: $existing" -ForegroundColor Yellow
            return $existing
        }
        if (-not $tools.MakeAppx) {
            throw "makeappx.exe was not found. Please install the Windows 10/11 SDK or add makeappx.exe to PATH to package the MSIX."
        }
        if (-not $tools.SignTool) {
            throw "signtool.exe was not found. Please install the Windows 10/11 SDK or add signtool.exe to PATH to sign the MSIX."
        }
    }

    Write-Host "Repackaging NetFlow.msix on your machine..." -ForegroundColor Cyan

    # If in a Cargo workspace repo, ensure latest release binary is built
    $repoRoot = (Resolve-Path (Join-Path $ScriptDir "..")).Path
    $cargoToml = Join-Path $repoRoot "Cargo.toml"
    $candRelease = Join-Path $repoRoot "target\release\net-flow.exe"
    $binExe = $null

    if (Test-Path $cargoToml) {
        $cargoCmd = Get-Command cargo -ErrorAction SilentlyContinue
        if ($cargoCmd) {
            Write-Host "Building latest release binary via cargo..." -ForegroundColor Cyan
            & cargo build --release --bin net-flow
            if ($LASTEXITCODE -ne 0) {
                throw "Failed to build net-flow.exe via cargo!"
            }
            if (Test-Path $candRelease) {
                $binExe = (Resolve-Path $candRelease).Path
            }
        }
    }

    # Locate binary if not built above
    if (-not $binExe -or -not (Test-Path $binExe)) {
        $binExe = Find-ExistingBinary
        if (-not $binExe) {
            throw "net-flow.exe not found! Please build the binary via cargo or ensure you extracted the full release archive."
        }
    }

    # Locate manifest
    $manifest = Get-PackageManifestPath
    if (-not $manifest -or -not (Test-Path $manifest)) {
        throw "AppxManifest.xml not found! Expected at $ManifestPath or widget/Package.appxmanifest"
    }

    # Read manifest content to resolve Publisher and Identity
    $manifestContent = Get-Content $manifest -Raw
    $publisher = Get-PackagePublisher

    # Generate or reuse non-expired certificate in Cert:\CurrentUser\My
    $now = Get-Date
    $cert = Get-ChildItem Cert:\CurrentUser\My -ErrorAction SilentlyContinue |
        Where-Object { $_.Subject -eq $publisher -and $_.HasPrivateKey -and $_.NotAfter -gt $now } |
        Sort-Object NotAfter -Descending |
        Select-Object -First 1

    if (-not $cert) {
        Write-Host "Generating local signing certificate for $publisher..." -ForegroundColor Cyan
        $cert = New-SelfSignedCertificate `
            -Type Custom `
            -Subject $publisher `
            -KeyUsage DigitalSignature `
            -FriendlyName "Net Flow Sideload Signing" `
            -CertStoreLocation "Cert:\CurrentUser\My" `
            -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")
    } else {
        Write-Host "Reusing active local signing certificate (Expires: $($cert.NotAfter.ToShortDateString()))." -ForegroundColor Green
    }

    # Trust certificate in LocalMachine\TrustedPeople
    Install-SideloadCert -Certificate $cert

    # Prepare temporary layout
    $tempLayout = Join-Path $env:TEMP ("netflow_layout_" + [guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Force -Path $tempLayout | Out-Null

    try {
        Copy-Item $binExe (Join-Path $tempLayout "net-flow.exe") -Force

        $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText((Join-Path $tempLayout "AppxManifest.xml"), $manifestContent, $utf8NoBom)

        $assetsSrc = Join-Path $ScriptDir "Assets"
        if (-not (Test-Path $assetsSrc)) { $assetsSrc = Join-Path $ScriptDir "..\widget\Assets" }
        if (Test-Path $assetsSrc) {
            Copy-Item $assetsSrc (Join-Path $tempLayout "Assets") -Recurse -Force
        }

        New-Item -ItemType Directory -Force -Path (Join-Path $tempLayout "Public") | Out-Null

        # PRI file
        $priSrc = Join-Path $ScriptDir "resources.pri"
        if (-not (Test-Path $priSrc)) {
            $candPri = Join-Path $ScriptDir "..\target\msix\resources.pri"
            if (Test-Path $candPri) { $priSrc = $candPri }
        }
        if (Test-Path $priSrc) {
            Copy-Item $priSrc (Join-Path $tempLayout "resources.pri") -Force
        } elseif ($tools.MakePri) {
            $priConfig = Join-Path $tempLayout "priconfig.xml"
            & $tools.MakePri createconfig /cf $priConfig /dq "en-US" /pv "10.0.0" /o | Out-Null
            & $tools.MakePri new /pr $tempLayout /cf $priConfig /of (Join-Path $tempLayout "resources.pri") /o | Out-Null
            Remove-Item $priConfig -Force -ErrorAction SilentlyContinue
        }

        # Pack MSIX
        $outputMsix = $DefaultMsix
        if (Test-Path $outputMsix) { Remove-Item $outputMsix -Force -ErrorAction SilentlyContinue }

        Write-Host "Packing MSIX container to $outputMsix..." -ForegroundColor Cyan
        & $tools.MakeAppx pack /d $tempLayout /p $outputMsix /o | Out-Null
        if ($LASTEXITCODE -ne 0) {
            throw "makeappx.exe failed with exit code $LASTEXITCODE"
        }

        # Sign MSIX
        Write-Host "Signing NetFlow.msix with certificate thumbprint $($cert.Thumbprint)..." -ForegroundColor Cyan
        & $tools.SignTool sign /fd SHA256 /sha1 $cert.Thumbprint $outputMsix | Out-Null
        if ($LASTEXITCODE -ne 0) {
            throw "signtool.exe failed to sign $outputMsix (exit code $LASTEXITCODE)"
        }

        Write-Host "MSIX packaged and signed successfully: $outputMsix" -ForegroundColor Green
        return $outputMsix
    } finally {
        if (Test-Path $tempLayout) {
            Remove-Item $tempLayout -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

function Get-Status {
    Show-Header
    Write-Host "Location        : $ScriptDir"

    $msix = Find-ExistingMsix
    if ($msix) {
        $msixSize = [math]::Round(((Get-Item $msix).Length / 1MB), 2)
        Write-Host "MSIX Package    : [FOUND] $msix ($msixSize MB)" -ForegroundColor Green
    } else {
        Write-Host "MSIX Package    : [NOT FOUND]" -ForegroundColor Yellow
    }

    $bin = Find-ExistingBinary
    Write-Host "Executable Found: $(if ($bin) { '[YES] ' + $bin } else { '[NO]' })" -ForegroundColor $(if ($bin) { 'Green' } else { 'Gray' })

    $manifest = Get-PackageManifestPath
    Write-Host "Manifest Found  : $(if ($manifest) { '[YES] ' + $manifest } else { '[NO]' })" -ForegroundColor $(if ($manifest) { 'Green' } else { 'Gray' })

    $identityName = Get-PackageIdentity
    $appx = Get-AppxPackage -Name $identityName -ErrorAction SilentlyContinue
    if ($appx) {
        Write-Host "Widget Package  : [INSTALLED] $($appx.PackageFullName)" -ForegroundColor Green
        Write-Host "Install Location: $($appx.InstallLocation)" -ForegroundColor Gray
    } else {
        Write-Host "Widget Package  : [NOT INSTALLED] (Identity: $identityName)" -ForegroundColor Yellow
    }

    $runningProc = Get-Process -Name "net-flow" -ErrorAction SilentlyContinue
    Write-Host "Widget Process  : $(if ($runningProc) { '[RUNNING] (PID: ' + $runningProc.Id + ')' } else { '[IDLE]' })" -ForegroundColor $(if ($runningProc) { 'Green' } else { 'Gray' })
    Write-Host ""
}

function Install-Package {
    Show-Header

    $msixToInstall = $null

    if ($MsixPath -and (Test-Path $MsixPath)) {
        Write-Host "Using explicitly specified MSIX: $MsixPath" -ForegroundColor Green
        $msixToInstall = (Resolve-Path $MsixPath).Path
    } elseif (-not $Rebuild) {
        $existing = Find-ExistingMsix
        if ($existing) {
            Write-Host "Found existing package: $existing" -ForegroundColor Green
            Write-Host "Using existing MSIX (pass -Rebuild to compile and package fresh)." -ForegroundColor Cyan
            $msixToInstall = $existing
        }
    }

    if (-not $msixToInstall) {
        if ($Rebuild) {
            Write-Host "-Rebuild specified: Forcing repackage of NetFlow.msix..." -ForegroundColor Cyan
        }
        $msixToInstall = New-MsixPackage
    }

    if (-not $msixToInstall -or -not (Test-Path $msixToInstall)) {
        throw "Failed to locate or build NetFlow.msix for installation!"
    }

    # Verify signature and ensure signer certificate is trusted in LocalMachine\TrustedPeople
    $sig = Get-AuthenticodeSignature -FilePath $msixToInstall -ErrorAction SilentlyContinue
    if ($sig -and $sig.SignerCertificate) {
        Write-Host "Package signed by: $($sig.SignerCertificate.Subject)" -ForegroundColor Cyan
        Install-SideloadCert -Certificate $sig.SignerCertificate
    }

    Write-Host "Installing Net Flow MSIX package: $msixToInstall" -ForegroundColor Cyan

    # Stop any active Net Flow instance
    Write-Host "Stopping any running Net Flow processes..." -ForegroundColor Cyan
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 300

    # Remove previous package registration specifically by identity name
    $identityName = Get-PackageIdentity
    $existingAppx = Get-AppxPackage -Name $identityName -ErrorAction SilentlyContinue
    if ($existingAppx) {
        Write-Host "Removing previous package registration ($($existingAppx.PackageFullName))..." -ForegroundColor Cyan
        Remove-AppxPackage -Package $existingAppx.PackageFullName -ErrorAction SilentlyContinue
        Start-Sleep -Milliseconds 400
    }

    # Install package via Add-AppxPackage -Path
    Write-Host "Installing Net Flow package into Windows..." -ForegroundColor Cyan
    try {
        Add-AppxPackage -Path $msixToInstall -ForceApplicationShutdown
        Write-Host "Net Flow package installed successfully!" -ForegroundColor Green
    } catch {
        Write-Host "Installation failed: $_" -ForegroundColor Red
        Write-Host ""
        Write-Host "Widget sideloading failed. For local development:" -ForegroundColor Yellow
        Write-Host "  1. Enable Developer Mode in Windows Settings (Settings -> System -> For developers -> Developer Mode)." -ForegroundColor Yellow
        Write-Host "  2. Ensure the package signing certificate is trusted in Local Computer -> Trusted People." -ForegroundColor Yellow
        Write-Host "  3. Run this installer again." -ForegroundColor Yellow
        Write-Host ""
        throw $_
    }

    Restart-WidgetBoard
}

function Remove-SideloadCert {
    param([string]$Subject = "")

    if (-not $Subject) {
        $Subject = Get-PackagePublisher
    }

    Write-Host "Removing Net Flow signing certificates ($Subject)..." -ForegroundColor Yellow

    # CurrentUser stores
    Get-ChildItem Cert:\CurrentUser\My, Cert:\CurrentUser\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Subject -eq $Subject -or $_.FriendlyName -eq "Net Flow Sideload Signing" } |
        Remove-Item -Force -ErrorAction SilentlyContinue

    # LocalMachine\TrustedPeople (and legacy Root cleanup if friendly name matches)
    $hasMachineCerts = Get-ChildItem Cert:\LocalMachine\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Subject -eq $Subject -or $_.FriendlyName -eq "Net Flow Sideload Signing" }
    $hasLegacyRoot = Get-ChildItem Cert:\LocalMachine\Root -ErrorAction SilentlyContinue |
        Where-Object { $_.Subject -eq $Subject -and $_.FriendlyName -eq "Net Flow Sideload Signing" }

    if ($hasMachineCerts -or $hasLegacyRoot) {
        try {
            if ($hasMachineCerts) { $hasMachineCerts | Remove-Item -Force -ErrorAction Stop }
            if ($hasLegacyRoot) { $hasLegacyRoot | Remove-Item -Force -ErrorAction SilentlyContinue }
            Write-Host "Certificate removed from LocalMachine\TrustedPeople." -ForegroundColor Green
        } catch {
            Write-Host "Prompting for elevation to remove certificate from LocalMachine stores..." -ForegroundColor Cyan
            $cmd = "Get-ChildItem Cert:\LocalMachine\TrustedPeople, Cert:\LocalMachine\Root -ErrorAction SilentlyContinue | Where-Object { `$_.Subject -eq '$Subject' -or `$_.FriendlyName -eq 'Net Flow Sideload Signing' } | Remove-Item -Force"
            Start-Process powershell -Verb RunAs -Wait -ArgumentList "-NoProfile -ExecutionPolicy Bypass -Command `"$cmd`""
        }
    }

    Write-Host "Net Flow signing certificates removed." -ForegroundColor Green
}

function Uninstall-Package {
    Show-Header
    Write-Host "Stopping any running Net Flow processes..." -ForegroundColor Cyan
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue

    $identityName = Get-PackageIdentity
    $appx = Get-AppxPackage -Name $identityName -ErrorAction SilentlyContinue
    if ($appx) {
        Write-Host "Removing Net Flow widget package ($($appx.PackageFullName)) from Windows..." -ForegroundColor Yellow
        Remove-AppxPackage -Package $appx.PackageFullName -ErrorAction SilentlyContinue
        Write-Host "Net Flow widget uninstalled successfully." -ForegroundColor Green
    } else {
        Write-Host "Package '$identityName' is not currently installed." -ForegroundColor Yellow
    }

    Remove-SideloadCert

    Restart-WidgetBoard
}

function Restart-WidgetBoard {
    Write-Host "Refreshing Windows 11 Widgets Board processes (best-effort)..." -ForegroundColor Cyan
    foreach ($procName in @("WidgetBoard", "WidgetService", "Widgets")) {
        Stop-Process -Name $procName -Force -ErrorAction SilentlyContinue
    }
    Start-Sleep -Milliseconds 600
    Write-Host "Widgets Board refreshed! Press Win + W to open your Widgets Board." -ForegroundColor Green
}

# Main Execution Dispatch
if ($Status) {
    Get-Status
}
elseif ($Uninstall) {
    Uninstall-Package
}
elseif ($RemoveCert) {
    Show-Header
    Remove-SideloadCert
}
elseif ($RestartWidgets) {
    Restart-WidgetBoard
}
else {
    # Default behavior: Install existing package (or rebuild if -Rebuild specified or missing)
    Install-Package
    Write-Host ""
    Get-Status
}
