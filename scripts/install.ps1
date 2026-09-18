# Net Flow - Windows 11 Widget Installer Helper
# Automatically trusts the sideloading certificate and installs NetFlow.msix into Windows.

[CmdletBinding()]
param(
    [switch]$Install,
    [switch]$Uninstall,
    [switch]$Status,
    [switch]$RestartWidgets,
    [string]$MsixPath = "",
    [string]$CertPath = ""
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$DefaultMsix = Join-Path $ScriptDir "NetFlow.msix"
$DefaultCer = Join-Path $ScriptDir "NetFlow_Sideload_Cert.cer"

function Show-Header {
    Write-Host "=================================================" -ForegroundColor Cyan
    Write-Host "  Net Flow - Windows 11 Widget Installer" -ForegroundColor Cyan
    Write-Host "=================================================" -ForegroundColor Cyan
}

function Find-MsixFile {
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
    param([string]$MsixFile = "")

    $cer = if ($CertPath -and (Test-Path $CertPath)) { $CertPath } else { $DefaultCer }
    if (-not (Test-Path $cer) -and $MsixFile) {
        $msixDir = Split-Path -Parent $MsixFile
        $candCer = Join-Path $msixDir "NetFlow_Sideload_Cert.cer"
        if (Test-Path $candCer) { $cer = $candCer }
    }

    $certObj = $null
    if (Test-Path $cer) {
        try {
            $certObj = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($cer)
        } catch {}
    }

    # Extract certificate from signed MSIX if separate .cer file is missing
    if (-not $certObj -and $MsixFile -and (Test-Path $MsixFile)) {
        try {
            $sig = Get-AuthenticodeSignature -FilePath $MsixFile -ErrorAction SilentlyContinue
            if ($sig -and $sig.SignerCertificate) {
                $certObj = $sig.SignerCertificate
            }
        } catch {}
    }

    if (-not $certObj) {
        return
    }

    $thumb = $certObj.Thumbprint
    $inTrustedPeople = Get-ChildItem Cert:\CurrentUser\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Thumbprint -eq $thumb }
    $inRoot = Get-ChildItem Cert:\LocalMachine\Root -ErrorAction SilentlyContinue |
        Where-Object { $_.Thumbprint -eq $thumb }

    if (-not $inTrustedPeople -and -not $inRoot) {
        Write-Host "Adding package certificate ($($certObj.Subject)) to Cert:\CurrentUser\TrustedPeople..." -ForegroundColor Cyan
        try {
            $store = New-Object System.Security.Cryptography.X509Certificates.X509Store("TrustedPeople", "CurrentUser")
            $store.Open([System.Security.Cryptography.X509Certificates.OpenFlags]::ReadWrite)
            $store.Add($certObj)
            $store.Close()
            Write-Host "Certificate trusted successfully." -ForegroundColor Green
        } catch {
            Write-Warning "Could not automatically import certificate to TrustedPeople: $_"
        }
    } else {
        Write-Host "Package signing certificate is trusted." -ForegroundColor Green
    }
}

function Get-Status {
    Show-Header
    Write-Host "Location        : $ScriptDir"

    $msix = Find-MsixFile
    if ($msix) {
        $msixSize = [math]::Round(((Get-Item $msix).Length / 1MB), 2)
        Write-Host "MSIX Package    : [FOUND] $msix ($msixSize MB)" -ForegroundColor Green
    } else {
        Write-Host "MSIX Package    : [NOT FOUND]" -ForegroundColor Yellow
    }

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

    $msixToInstall = Find-MsixFile
    if (-not $msixToInstall -or -not (Test-Path $msixToInstall)) {
        throw "NetFlow.msix not found in $ScriptDir! Please ensure NetFlow.msix is in the same directory."
    }

    Write-Host "Installing Net Flow MSIX package: $msixToInstall" -ForegroundColor Cyan

    # Ensure certificate trust
    Install-SideloadCert -MsixFile $msixToInstall

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

function Uninstall-Package {
    Show-Header
    Write-Host "Stopping any running Net Flow processes..." -ForegroundColor Cyan
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue

    Write-Host "Removing Net Flow widget package from Windows..." -ForegroundColor Yellow
    Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue | Remove-AppxPackage -ErrorAction SilentlyContinue
    Write-Host "Net Flow widget uninstalled successfully." -ForegroundColor Green
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
elseif ($RestartWidgets) {
    Restart-WidgetBoard
}
else {
    # Default behavior: Install package
    Install-Package
    Write-Host ""
    Get-Status
}
