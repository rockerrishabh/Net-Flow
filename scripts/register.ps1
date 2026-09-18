# Net Flow - Windows 11 Widget Registration & MSIX Installer Helper
# Automatically packages Net Flow into an MSIX container (if not already built),
# trusts the signing certificate, and installs the package into Windows 11.

[CmdletBinding()]
param(
    [switch]$Register,
    [switch]$Unregister,
    [switch]$InstallCert,
    [switch]$Status,
    [switch]$RestartWidgets,
    [switch]$Rebuild,
    [string]$MsixPath = ""
)

$ErrorActionPreference = "Stop"
$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ExePath = Join-Path $ScriptDir "net-flow.exe"
$ManifestPath = Join-Path $ScriptDir "AppxManifest.xml"
$DefaultMsix = Join-Path $ScriptDir "NetFlow.msix"
$DefaultCer = Join-Path $ScriptDir "NetFlow_Sideload_Cert.cer"

function Show-Header {
    Write-Host "=================================================" -ForegroundColor Cyan
    Write-Host "  Net Flow - Windows 11 Widget Registration" -ForegroundColor Cyan
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
                $tools.MakePri = Join-Path $x64Path "makepri.exe"
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

function New-MsixPackage {
    if (-not $Rebuild) {
        $existing = Find-ExistingMsix
        if ($existing) {
            return $existing
        }
    }

    Write-Host "Creating NetFlow.msix package from layout..." -ForegroundColor Cyan

    # Locate Windows SDK tools
    $tools = Find-SdkTools
    if (-not $tools.MakeAppx) {
        throw "makeappx.exe was not found. Please install the Windows 10/11 SDK or add makeappx.exe to PATH to package the MSIX."
    }
    if (-not $tools.SignTool) {
        throw "signtool.exe was not found. Please install the Windows 10/11 SDK or add signtool.exe to PATH to sign the MSIX."
    }

    # Resolve executable and manifest
    $binExe = $ExePath
    if (-not (Test-Path $binExe)) {
        $candRelease = Join-Path $ScriptDir "..\target\release\net-flow.exe"
        if (Test-Path $candRelease) {
            $binExe = (Resolve-Path $candRelease).Path
        }
        else {
            throw "net-flow.exe was not found at $binExe! Build the binary or extract the release archive."
        }
    }

    $manifest = $ManifestPath
    if (-not (Test-Path $manifest)) {
        $candManifest = Join-Path $ScriptDir "..\widget\Package.appxmanifest"
        if (Test-Path $candManifest) {
            $manifest = (Resolve-Path $candManifest).Path
        }
        else {
            throw "AppxManifest.xml was not found at $manifest!"
        }
    }

    # Prepare temporary staging layout
    $tempLayout = Join-Path $env:TEMP ("netflow_layout_" + [guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Force -Path $tempLayout | Out-Null

    try {
        # 1. Copy binary
        Copy-Item $binExe (Join-Path $tempLayout "net-flow.exe") -Force

        # 2. Copy and prepare manifest
        $manifestContent = Get-Content $manifest -Raw
        $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText((Join-Path $tempLayout "AppxManifest.xml"), $manifestContent, $utf8NoBom)

        # 3. Copy Assets
        $assetsSrc = Join-Path $ScriptDir "Assets"
        if (-not (Test-Path $assetsSrc)) {
            $assetsSrc = Join-Path $ScriptDir "..\widget\Assets"
        }
        if (Test-Path $assetsSrc) {
            Copy-Item $assetsSrc (Join-Path $tempLayout "Assets") -Recurse -Force
        }

        # 4. Create AppExtension Public folder
        New-Item -ItemType Directory -Force -Path (Join-Path $tempLayout "Public") | Out-Null

        # 5. Resources PRI
        $priSrc = Join-Path $ScriptDir "resources.pri"
        if (-not (Test-Path $priSrc)) {
            $candPri = Join-Path $ScriptDir "..\target\msix\resources.pri"
            if (Test-Path $candPri) { $priSrc = $candPri }
        }

        if (Test-Path $priSrc) {
            Copy-Item $priSrc (Join-Path $tempLayout "resources.pri") -Force
        }
        elseif ($tools.MakePri) {
            $priConfig = Join-Path $tempLayout "priconfig.xml"
            & $tools.MakePri createconfig /cf $priConfig /dq "en-US" /pv "10.0.0" /o | Out-Null
            & $tools.MakePri new /pr $tempLayout /cf $priConfig /of (Join-Path $tempLayout "resources.pri") /o | Out-Null
            Remove-Item $priConfig -Force -ErrorAction SilentlyContinue
        }

        # 6. Pack MSIX with makeappx
        $outputMsix = $DefaultMsix
        if (Test-Path $outputMsix) {
            Remove-Item $outputMsix -Force -ErrorAction SilentlyContinue
        }

        Write-Host "Packing MSIX container to $outputMsix..." -ForegroundColor Cyan
        & $tools.MakeAppx pack /d $tempLayout /p $outputMsix /o | Out-Null
        if ($LASTEXITCODE -ne 0) {
            throw "makeappx.exe failed with exit code $LASTEXITCODE"
        }

        # 7. Code signing
        $publisher = "CN=Development"
        if ($manifestContent -match 'Publisher="([^"]+)"') {
            $publisher = $matches[1]
        }

        $cert = Get-ChildItem Cert:\CurrentUser\My -ErrorAction SilentlyContinue |
        Where-Object { $_.Subject -eq $publisher -and $_.HasPrivateKey } |
        Select-Object -First 1

        if (-not $cert) {
            Write-Host "Creating developer signing certificate for $publisher..." -ForegroundColor Cyan
            $cert = New-SelfSignedCertificate `
                -Type Custom `
                -Subject $publisher `
                -KeyUsage DigitalSignature `
                -FriendlyName "Net Flow Sideload Signing" `
                -CertStoreLocation "Cert:\CurrentUser\My" `
                -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")
        }

        # Export public certificate for sideloading
        [System.IO.File]::WriteAllBytes($DefaultCer, $cert.Export([System.Security.Cryptography.X509Certificates.X509ContentType]::Cert))

        # Ensure trusted in CurrentUser\TrustedPeople
        $inTrusted = Get-ChildItem Cert:\CurrentUser\TrustedPeople -ErrorAction SilentlyContinue |
        Where-Object { $_.Thumbprint -eq $cert.Thumbprint }
        if (-not $inTrusted) {
            $store = New-Object System.Security.Cryptography.X509Certificates.X509Store("TrustedPeople", "CurrentUser")
            $store.Open([System.Security.Cryptography.X509Certificates.OpenFlags]::ReadWrite)
            $store.Add($cert)
            $store.Close()
        }

        # Sign MSIX package
        Write-Host "Signing NetFlow.msix with certificate thumbprint $($cert.Thumbprint)..." -ForegroundColor Cyan
        & $tools.SignTool sign /fd SHA256 /sha1 $cert.Thumbprint $outputMsix | Out-Null
        if ($LASTEXITCODE -ne 0) {
            throw "signtool.exe failed to sign $outputMsix (exit code $LASTEXITCODE)"
        }

        Write-Host "MSIX package created and signed successfully: $outputMsix" -ForegroundColor Green
        return $outputMsix
    }
    finally {
        if (Test-Path $tempLayout) {
            Remove-Item $tempLayout -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

function Install-SideloadCert {
    param([string]$MsixFile = "")

    $cerPath = $DefaultCer
    if (-not (Test-Path $cerPath) -and $MsixFile) {
        $msixDir = Split-Path -Parent $MsixFile
        $candCer = Join-Path $msixDir "NetFlow_Sideload_Cert.cer"
        if (Test-Path $candCer) { $cerPath = $candCer }
    }

    $cert = $null
    if (Test-Path $cerPath) {
        try {
            $cert = New-Object System.Security.Cryptography.X509Certificates.X509Certificate2($cerPath)
        }
        catch {}
    }

    if (-not $cert -and $MsixFile -and (Test-Path $MsixFile)) {
        try {
            $sig = Get-AuthenticodeSignature -FilePath $MsixFile -ErrorAction SilentlyContinue
            if ($sig -and $sig.SignerCertificate) {
                $cert = $sig.SignerCertificate
            }
        }
        catch {}
    }

    if (-not $cert) {
        return
    }

    $thumb = $cert.Thumbprint
    $inTrustedPeople = Get-ChildItem Cert:\CurrentUser\TrustedPeople -ErrorAction SilentlyContinue | Where-Object { $_.Thumbprint -eq $thumb }
    $inRoot = Get-ChildItem Cert:\LocalMachine\Root -ErrorAction SilentlyContinue | Where-Object { $_.Thumbprint -eq $thumb }

    if (-not $inTrustedPeople -and -not $inRoot) {
        Write-Host "Adding package certificate ($($cert.Subject)) to Cert:\CurrentUser\TrustedPeople..." -ForegroundColor Cyan
        try {
            $store = New-Object System.Security.Cryptography.X509Certificates.X509Store("TrustedPeople", "CurrentUser")
            $store.Open([System.Security.Cryptography.X509Certificates.OpenFlags]::ReadWrite)
            $store.Add($cert)
            $store.Close()
            Write-Host "Certificate trusted successfully." -ForegroundColor Green
        }
        catch {
            Write-Warning "Could not automatically import certificate to TrustedPeople: $_"
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
    }
    else {
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
    }
    else {
        Write-Host "Widget Package  : [NOT INSTALLED]" -ForegroundColor Yellow
    }

    $runningProc = Get-Process -Name "net-flow" -ErrorAction SilentlyContinue
    Write-Host "Widget Process  : $(if ($runningProc) { '[RUNNING] (PID: ' + $runningProc.Id + ')' } else { '[IDLE]' })" -ForegroundColor $(if ($runningProc) { 'Green' } else { 'Gray' })
    Write-Host ""
}

function Register-Package {
    Show-Header

    $msixToInstall = New-MsixPackage
    if (-not $msixToInstall -or -not (Test-Path $msixToInstall)) {
        throw "Failed to locate or build NetFlow.msix for installation!"
    }

    Write-Host "Installing MSIX package: $msixToInstall" -ForegroundColor Cyan

    # Ensure certificate trust
    Install-SideloadCert -MsixFile $msixToInstall

    # Stop any active Net Flow instance
    Write-Host "Terminating any running Net Flow processes..." -ForegroundColor Cyan
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 300

    # Remove previous package registration
    Write-Host "Removing previous package registration if present..." -ForegroundColor Cyan
    Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue | Remove-AppxPackage -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 400

    # Install the package via Add-AppxPackage -Path
    Write-Host "Registering Net Flow MSIX package into Windows..." -ForegroundColor Cyan
    try {
        Add-AppxPackage -Path $msixToInstall -ForceApplicationShutdown
        Write-Host "Net Flow MSIX package installed and registered successfully!" -ForegroundColor Green
    }
    catch {
        Write-Host "Installation failed: $_" -ForegroundColor Red
        Write-Host "Tip: Sideloading requires developer mode or trusting the certificate." -ForegroundColor Yellow
        throw $_
    }

    Restart-WidgetBoard
}

function Unregister-Package {
    Show-Header
    Write-Host "Stopping any running Net Flow processes..." -ForegroundColor Cyan
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue

    Write-Host "Removing Net Flow widget package from Windows..." -ForegroundColor Yellow
    Get-AppxPackage "*NetFlow*" -ErrorAction SilentlyContinue | Remove-AppxPackage -ErrorAction SilentlyContinue
    Write-Host "Net Flow widget unregistered successfully." -ForegroundColor Green
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
elseif ($Unregister) {
    Unregister-Package
}
elseif ($InstallCert) {
    Show-Header
    $msix = Find-ExistingMsix
    Install-SideloadCert -MsixFile $msix
}
elseif ($RestartWidgets) {
    Restart-WidgetBoard
}
else {
    # Default behavior: Always package (if missing or rebuild requested) and install/register
    Register-Package
    Write-Host ""
    Get-Status
}
