# Net Flow - Windows 11 Widget Installer Helper
# Automatically generates a local signing certificate, packages NetFlow.msix from layout,
# trusts the certificate in machine root, and installs the package into Windows 11.

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

    $thumb = $Certificate.Thumbprint

    # 1. Ensure trusted in CurrentUser\TrustedPeople
    $inCurrentTrusted = Get-ChildItem Cert:\CurrentUser\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Thumbprint -eq $thumb }
    if (-not $inCurrentTrusted) {
        try {
            $store = New-Object System.Security.Cryptography.X509Certificates.X509Store("TrustedPeople", "CurrentUser")
            $store.Open([System.Security.Cryptography.X509Certificates.OpenFlags]::ReadWrite)
            $store.Add($Certificate)
            $store.Close()
        } catch {}
    }

    # 2. Check Machine Root / Machine TrustedPeople (required by AppXSVC to avoid 0x800B0109)
    $inLocalRoot = Get-ChildItem Cert:\LocalMachine\Root -ErrorAction SilentlyContinue |
        Where-Object { $_.Thumbprint -eq $thumb }
    $inLocalTrusted = Get-ChildItem Cert:\LocalMachine\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Thumbprint -eq $thumb }

    if (-not $inLocalRoot -and -not $inLocalTrusted) {
        $tempCer = Join-Path $env:TEMP ("netflow_cert_" + $thumb + ".cer")
        [System.IO.File]::WriteAllBytes($tempCer, $Certificate.Export([System.Security.Cryptography.X509Certificates.X509ContentType]::Cert))

        # Try direct import (works if running elevated)
        try {
            Import-Certificate -CertStoreLocation "Cert:\LocalMachine\Root" -FilePath $tempCer -ErrorAction Stop | Out-Null
            Import-Certificate -CertStoreLocation "Cert:\LocalMachine\TrustedPeople" -FilePath $tempCer -ErrorAction SilentlyContinue | Out-Null
            Write-Host "Signing certificate installed to LocalMachine\Root." -ForegroundColor Green
        } catch {
            Write-Host "Sideloading requires trusting the package certificate in LocalMachine\Root." -ForegroundColor Yellow
            Write-Host "Prompting for administrator approval to trust certificate..." -ForegroundColor Cyan
            try {
                $argList = "-NoProfile -ExecutionPolicy Bypass -Command `"Import-Certificate -CertStoreLocation 'Cert:\LocalMachine\Root' -FilePath '$tempCer'; Import-Certificate -CertStoreLocation 'Cert:\LocalMachine\TrustedPeople' -FilePath '$tempCer'`""
                $proc = Start-Process powershell -Verb RunAs -Wait -PassThru -ArgumentList $argList
                if ($proc.ExitCode -eq 0) {
                    Write-Host "Certificate installed to LocalMachine\Root successfully." -ForegroundColor Green
                }
            } catch {
                Write-Warning "Could not elevate to install certificate. If installation fails, right-click install.ps1 -> Run with PowerShell as Administrator."
            }
        } finally {
            Remove-Item $tempCer -Force -ErrorAction SilentlyContinue
        }
    } else {
        Write-Host "Package signing certificate is trusted in machine root." -ForegroundColor Green
    }
}

function New-MsixPackage {
    if (-not $Rebuild) {
        $existing = Find-ExistingMsix
        if ($existing) {
            return $existing
        }
    }

    Write-Host "Packaging NetFlow.msix on your machine..." -ForegroundColor Cyan

    $tools = Find-SdkTools
    if (-not $tools.MakeAppx) {
        throw "makeappx.exe was not found. Please install the Windows 10/11 SDK or add makeappx.exe to PATH to package the MSIX."
    }
    if (-not $tools.SignTool) {
        throw "signtool.exe was not found. Please install the Windows 10/11 SDK or add signtool.exe to PATH to sign the MSIX."
    }

    # Locate binary
    $binExe = $ExePath
    if (-not (Test-Path $binExe)) {
        $candRelease = Join-Path $ScriptDir "..\target\release\net-flow.exe"
        if (Test-Path $candRelease) { $binExe = (Resolve-Path $candRelease).Path }
        else { throw "net-flow.exe not found at $binExe! Please ensure you extracted the full release archive." }
    }

    # Locate manifest
    $manifest = $ManifestPath
    if (-not (Test-Path $manifest)) {
        $candManifest = Join-Path $ScriptDir "..\widget\Package.appxmanifest"
        if (Test-Path $candManifest) { $manifest = (Resolve-Path $candManifest).Path }
        else { throw "AppxManifest.xml not found at $manifest!" }
    }

    # Read manifest content to resolve Publisher
    $manifestContent = Get-Content $manifest -Raw
    $publisher = "CN=Development"
    if ($manifestContent -match 'Publisher="([^"]+)"') {
        $publisher = $matches[1]
    }

    # Generate or reuse certificate in Cert:\CurrentUser\My
    $cert = Get-ChildItem Cert:\CurrentUser\My -ErrorAction SilentlyContinue |
        Where-Object { $_.Subject -eq $publisher -and $_.HasPrivateKey } |
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
    }

    # Trust certificate in machine root
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

    $exeExists = Test-Path $ExePath
    Write-Host "Executable Found: $(if ($exeExists) { '[YES]' } else { '[NO]' })" -ForegroundColor $(if ($exeExists) { 'Green' } else { 'Gray' })

    $manifestExists = Test-Path $ManifestPath
    Write-Host "Manifest Found  : $(if ($manifestExists) { '[YES]' } else { '[NO]' })" -ForegroundColor $(if ($manifestExists) { 'Green' } else { 'Gray' })

    $appx = Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue
    if ($appx) {
        Write-Host "Widget Package  : [INSTALLED] $($appx.PackageFullName)" -ForegroundColor Green
        Write-Host "Install Location: $($appx.InstallLocation)" -ForegroundColor Gray
    } else {
        Write-Host "Widget Package  : [NOT INSTALLED]" -ForegroundColor Yellow
    }

    $runningProc = Get-Process -Name "net-flow" -ErrorAction SilentlyContinue
    Write-Host "Widget Process  : $(if ($runningProc) { '[RUNNING] (PID: ' + $runningProc.Id + ')' } else { '[IDLE]' })" -ForegroundColor $(if ($runningProc) { 'Green' } else { 'Gray' })
    Write-Host ""
}

function Install-Package {
    Show-Header

    $msixToInstall = New-MsixPackage
    if (-not $msixToInstall -or -not (Test-Path $msixToInstall)) {
        throw "Failed to locate or build NetFlow.msix for installation!"
    }

    Write-Host "Installing Net Flow MSIX package: $msixToInstall" -ForegroundColor Cyan

    # Stop any active Net Flow instance
    Write-Host "Stopping any running Net Flow processes..." -ForegroundColor Cyan
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 300

    # Remove previous package registration
    Write-Host "Removing previous package registration if present..." -ForegroundColor Cyan
    Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue | Remove-AppxPackage -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 400

    # Install package via Add-AppxPackage -Path
    Write-Host "Installing Net Flow package into Windows..." -ForegroundColor Cyan
    try {
        Add-AppxPackage -Path $msixToInstall -ForceApplicationShutdown
        Write-Host "Net Flow package installed successfully!" -ForegroundColor Green
    } catch {
        Write-Host "Installation failed: $_" -ForegroundColor Red
        Write-Host "Tip: Sideloading requires developer mode or trusting the certificate." -ForegroundColor Yellow
        throw $_
    }

    Restart-WidgetBoard
}

function Remove-SideloadCert {
    param([string]$Subject = "CN=Development")

    if (Test-Path $ManifestPath) {
        $m = Get-Content $ManifestPath -Raw
        if ($m -match 'Publisher="([^"]+)"') { $Subject = $matches[1] }
    }

    Write-Host "Removing Net Flow signing certificates ($Subject)..." -ForegroundColor Yellow

    # CurrentUser stores
    Get-ChildItem Cert:\CurrentUser\My, Cert:\CurrentUser\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Subject -eq $Subject -or $_.FriendlyName -eq "Net Flow Sideload Signing" } |
        Remove-Item -Force -ErrorAction SilentlyContinue

    # LocalMachine stores (Root & TrustedPeople)
    $hasMachineCerts = Get-ChildItem Cert:\LocalMachine\Root, Cert:\LocalMachine\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Subject -eq $Subject -or $_.FriendlyName -eq "Net Flow Sideload Signing" }

    if ($hasMachineCerts) {
        try {
            $hasMachineCerts | Remove-Item -Force -ErrorAction Stop
            Write-Host "Certificate removed from LocalMachine stores." -ForegroundColor Green
        } catch {
            Write-Host "Prompting for elevation to remove certificate from LocalMachine stores..." -ForegroundColor Cyan
            $cmd = "Get-ChildItem Cert:\LocalMachine\Root, Cert:\LocalMachine\TrustedPeople -ErrorAction SilentlyContinue | Where-Object { `$_.Subject -eq '$Subject' -or `$_.FriendlyName -eq 'Net Flow Sideload Signing' } | Remove-Item -Force"
            Start-Process powershell -Verb RunAs -Wait -ArgumentList "-NoProfile -ExecutionPolicy Bypass -Command `"$cmd`""
        }
    }

    Write-Host "Net Flow signing certificates removed." -ForegroundColor Green
}

function Uninstall-Package {
    Show-Header
    Write-Host "Stopping any running Net Flow processes..." -ForegroundColor Cyan
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue

    Write-Host "Removing Net Flow widget package from Windows..." -ForegroundColor Yellow
    Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue | Remove-AppxPackage -ErrorAction SilentlyContinue
    Write-Host "Net Flow widget uninstalled successfully." -ForegroundColor Green

    Remove-SideloadCert

    Restart-WidgetBoard
}

function Restart-WidgetBoard {
    Write-Host "Refreshing Windows 11 Widgets Board processes..." -ForegroundColor Cyan
    Stop-Process -Name "WidgetBoard", "WidgetService", "Widgets" -Force -ErrorAction SilentlyContinue
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
    # Default behavior: Package (if missing or rebuild requested) and install
    Install-Package
    Write-Host ""
    Get-Status
}
