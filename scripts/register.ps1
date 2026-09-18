# Net Flow - Portable / Standalone Registration Helper
# Enables running and registering the Net Flow widget provider directly from the portable archive.

param(
    [switch]$RegisterCOM,
    [switch]$UnregisterCOM,
    [switch]$InstallCert,
    [switch]$Status,
    [switch]$RestartWidgets
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ExePath = Join-Path $ScriptDir "net-flow.exe"
$Clsid = "{A8E4C976-3F5D-4B2E-9C1A-7D6E8F0B2A4C}"
$RegKey = "HKCU:\Software\Classes\CLSID\$Clsid"

function Show-Header {
    Write-Host "=================================================" -ForegroundColor Cyan
    Write-Host " 📊 Net Flow - Portable Registration Helper" -ForegroundColor Cyan
    Write-Host "=================================================" -ForegroundColor Cyan
}

function Get-Status {
    Show-Header
    Write-Host "Executable Path : $ExePath"
    $exeExists = Test-Path $ExePath
    Write-Host "Executable Found: $(if ($exeExists) { '[YES]' } else { '[NO]' })" -ForegroundColor $(if ($exeExists) { 'Green' } else { 'Red' })

    $isComRegistered = Test-Path $RegKey
    Write-Host "COM Registered  : $(if ($isComRegistered) { '[YES]' } else { '[NO]' })" -ForegroundColor $(if ($isComRegistered) { 'Green' } else { 'Yellow' })
    if ($isComRegistered) {
        $serverPath = (Get-ItemProperty -Path "$RegKey\LocalServer32" -ErrorAction SilentlyContinue).'(default)'
        Write-Host "Registered Path : $serverPath" -ForegroundColor DarkGray
    }

    $appx = Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue
    Write-Host "MSIX Installed  : $(if ($appx) { '[YES] ' + $appx.PackageFullName } else { '[NO]' })" -ForegroundColor $(if ($appx) { 'Green' } else { 'Gray' })

    $runningProc = Get-Process -Name "net-flow" -ErrorAction SilentlyContinue
    Write-Host "Process Running : $(if ($runningProc) { '[RUNNING] (PID: ' + $runningProc.Id + ')' } else { '[STOPPED]' })" -ForegroundColor $(if ($runningProc) { 'Green' } else { 'Gray' })
    Write-Host ""
}

function Register-ComServer {
    Show-Header
    if (-not (Test-Path $ExePath)) {
        throw "net-flow.exe not found at $ExePath!"
    }

    Write-Host "Registering COM LocalServer32 in HKCU..." -ForegroundColor Cyan
    New-Item -Path $RegKey -Force | Out-Null
    Set-ItemProperty -Path $RegKey -Name "(default)" -Value "Net Flow Widget COM Provider"
    
    $localServerKey = "$RegKey\LocalServer32"
    New-Item -Path $localServerKey -Force | Out-Null
    Set-ItemProperty -Path $localServerKey -Name "(default)" -Value "`"$ExePath`""

    Write-Host "COM class $Clsid registered successfully to: $ExePath" -ForegroundColor Green
    Restart-WidgetBoard
}

function Unregister-ComServer {
    Show-Header
    if (Test-Path $RegKey) {
        Write-Host "Removing COM registration: $RegKey..." -ForegroundColor Yellow
        Remove-Item -Path $RegKey -Recurse -Force
        Write-Host "COM registration removed." -ForegroundColor Green
    }
    else {
        Write-Host "No HKCU COM registration found for $Clsid." -ForegroundColor Gray
    }
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
    Write-Host "Widgets Board refreshed. Press Win + W to open your Widgets Board." -ForegroundColor Green
}

# Main Execution Dispatch
if ($RegisterCOM) {
    Register-ComServer
}
elseif ($UnregisterCOM) {
    Unregister-ComServer
}
elseif ($InstallCert) {
    Install-SideloadCert
}
elseif ($RestartWidgets) {
    Restart-WidgetBoard
}
else {
    Get-Status
    Write-Host "Usage options:" -ForegroundColor White
    Write-Host "  .\register.ps1 -RegisterCOM     Register local net-flow.exe as COM widget provider"
    Write-Host "  .\register.ps1 -UnregisterCOM   Remove COM registration"
    Write-Host "  .\register.ps1 -InstallCert     Install bundled public certificate for sideloading"
    Write-Host "  .\register.ps1 -RestartWidgets  Restart Windows Widgets Board service"
    Write-Host "  .\register.ps1 -Status          Display current registration and process state"
}
