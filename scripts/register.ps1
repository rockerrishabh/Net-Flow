# Net Flow - Portable / Standalone Registration Helper
# Registers the Net Flow widget provider directly into the Windows 11 Widgets Board.

param(
    [switch]$Register,
    [switch]$Unregister,
    [switch]$InstallCert,
    [switch]$Status,
    [switch]$RestartWidgets
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ExePath = Join-Path $ScriptDir "net-flow.exe"
$ManifestPath = Join-Path $ScriptDir "AppxManifest.xml"

function Show-Header {
    Write-Host "=================================================" -ForegroundColor Cyan
    Write-Host " 📊 Net Flow - Windows 11 Widget Registration" -ForegroundColor Cyan
    Write-Host "=================================================" -ForegroundColor Cyan
}

function Get-Status {
    Show-Header
    Write-Host "Location        : $ScriptDir"
    $exeExists = Test-Path $ExePath
    Write-Host "Executable Found: $(if ($exeExists) { '[YES]' } else { '[NO]' })" -ForegroundColor $(if ($exeExists) { 'Green' } else { 'Red' })

    $manifestExists = Test-Path $ManifestPath
    Write-Host "Manifest Found  : $(if ($manifestExists) { '[YES]' } else { '[NO]' })" -ForegroundColor $(if ($manifestExists) { 'Green' } else { 'Red' })

    $appx = Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue
    Write-Host "Widget Package  : $(if ($appx) { '[REGISTERED] ' + $appx.PackageFullName } else { '[NOT REGISTERED]' })" -ForegroundColor $(if ($appx) { 'Green' } else { 'Yellow' })

    $runningProc = Get-Process -Name "net-flow" -ErrorAction SilentlyContinue
    Write-Host "Widget Process  : $(if ($runningProc) { '[RUNNING] (PID: ' + $runningProc.Id + ')' } else { '[IDLE]' })" -ForegroundColor $(if ($runningProc) { 'Green' } else { 'Gray' })
    Write-Host ""
}

function Register-Package {
    Show-Header
    if (-not (Test-Path $ManifestPath)) {
        throw "AppxManifest.xml not found in $ScriptDir! Required for Windows Widgets Board registration."
    }

    Write-Host "Registering Net Flow widget in Windows 11 Widgets Board..." -ForegroundColor Cyan
    Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue | Remove-AppxPackage -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 400

    Add-AppxPackage -Register $ManifestPath -ForceApplicationShutdown
    Write-Host "Net Flow widget package registered successfully!" -ForegroundColor Green
    Restart-WidgetBoard
}

function Unregister-Package {
    Show-Header
    Write-Host "Removing Net Flow widget registration..." -ForegroundColor Yellow
    Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue | Remove-AppxPackage -ErrorAction SilentlyContinue
    Write-Host "Net Flow widget unregistered." -ForegroundColor Green
    Restart-WidgetBoard
}

function Install-SideloadCert {
    Show-Header
    $cerPath = Join-Path $ScriptDir "NetFlow_Sideload_Cert.cer"
    if (-not (Test-Path $cerPath)) {
        Write-Host "Certificate file not found at $cerPath." -ForegroundColor Yellow
        return
    }

    Write-Host "Installing certificate to Cert:\CurrentUser\TrustedPeople..." -ForegroundColor Cyan
    Import-Certificate -CertStoreLocation "Cert:\CurrentUser\TrustedPeople" -FilePath $cerPath | Out-Null
    Write-Host "Certificate installed successfully." -ForegroundColor Green
}

function Restart-WidgetBoard {
    Write-Host "Restarting Windows Widgets Board processes..." -ForegroundColor Cyan
    Stop-Process -Name "WidgetBoard", "WidgetService", "Widgets" -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 600
    Write-Host "Widgets Board refreshed! Press Win + W to open your Widgets Board." -ForegroundColor Green
}

# Main Execution Dispatch
if ($Register) {
    Register-Package
}
elseif ($Unregister) {
    Unregister-Package
}
elseif ($InstallCert) {
    Install-SideloadCert
}
elseif ($RestartWidgets) {
    Restart-WidgetBoard
}
else {
    # Default behavior: If not registered, prompt to register; if registered, show status
    Get-Status
    $appx = Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue
    if (-not $appx) {
        Write-Host "Net Flow is not registered yet in your Widgets Board." -ForegroundColor Yellow
        Write-Host "Registering now..." -ForegroundColor Cyan
        Register-Package
    } else {
        Write-Host "Usage commands:" -ForegroundColor White
        Write-Host "  .\register.ps1 -Register        Register this folder in Windows Widgets Board"
        Write-Host "  .\register.ps1 -Unregister      Remove widget registration"
        Write-Host "  .\register.ps1 -RestartWidgets  Restart Windows Widgets Board service"
        Write-Host "  .\register.ps1 -Status          Display current registration and process state"
    }
}
