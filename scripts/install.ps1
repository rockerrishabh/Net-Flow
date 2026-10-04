# Net Flow - Windows 11 Widget Installer Helper
# Automatically generates a local signing certificate, packages NetFlow.msix from layout,
# trusts the certificate in Local Computer -> Trusted People, and installs the package into Windows 11.

[CmdletBinding()]
param(
    [switch]$Install,
    [switch]$Uninstall,
    [switch]$PurgeData,
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
$InstallerStatePath = Join-Path (Join-Path $env:LOCALAPPDATA "NetFlow") "installer-trusted-cert-thumbprints.txt"
$script:GeneratedMsixPath = $null
$script:GeneratedSigningCertificate = $null

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

function Read-XmlFile {
    param([Parameter(Mandatory)][string]$Path)
    $settings = New-Object System.Xml.XmlReaderSettings
    $settings.DtdProcessing = [System.Xml.DtdProcessing]::Prohibit
    $settings.XmlResolver = $null
    $reader = [System.Xml.XmlReader]::Create($Path, $settings)
    try {
        $document = New-Object System.Xml.XmlDocument
        $document.XmlResolver = $null
        $document.Load($reader)
        return $document
    }
    finally { $reader.Dispose() }
}

function Get-MsixMetadata {
    param([Parameter(Mandatory)][string]$Path)
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $archive = [System.IO.Compression.ZipFile]::OpenRead($Path)
    try {
        $entry = $archive.GetEntry("AppxManifest.xml")
        if (-not $entry) { throw "The package does not contain a root AppxManifest.xml." }
        $stream = $entry.Open()
        try {
            $reader = [System.Xml.XmlReader]::Create($stream, (New-Object System.Xml.XmlReaderSettings))
            try {
                $document = New-Object System.Xml.XmlDocument
                $document.XmlResolver = $null
                $document.Load($reader)
            }
            finally { $reader.Dispose() }
        }
        finally { $stream.Dispose() }
        $identity = $document.SelectSingleNode("//*[local-name()='Identity']")
        if (-not $identity) { throw "The package manifest has no Identity element." }
        return [pscustomobject]@{
            Name = $identity.GetAttribute("Name")
            Publisher = $identity.GetAttribute("Publisher")
            Architecture = $identity.GetAttribute("ProcessorArchitecture")
        }
    }
    finally { $archive.Dispose() }
}

function Get-PeArchitecture {
    param([Parameter(Mandatory)][string]$Path)
    $stream = [System.IO.File]::OpenRead($Path)
    try {
        $reader = New-Object System.IO.BinaryReader($stream)
        $stream.Position = 0x3C
        $peOffset = $reader.ReadInt32()
        $stream.Position = $peOffset + 4
        switch ($reader.ReadUInt16()) {
            0x8664 { return "x64" }
            0xAA64 { return "arm64" }
            0x014c { return "x86" }
            default { throw "Unsupported executable architecture in $Path." }
        }
    }
    finally { $stream.Dispose() }
}

function Get-TrackedCertThumbprints {
    if (Test-Path $InstallerStatePath) {
        return @(Get-Content $InstallerStatePath -ErrorAction SilentlyContinue |
            ForEach-Object { $_.Trim().ToUpperInvariant() } | Where-Object { $_ -match '^[A-F0-9]{40}$' } | Select-Object -Unique)
    }
    return @()
}

function Save-TrackedCertThumbprint {
    param([Parameter(Mandatory)][string]$Thumbprint)
    $thumbs = @(Get-TrackedCertThumbprints) + $Thumbprint.ToUpperInvariant()
    $directory = Split-Path -Parent $InstallerStatePath
    New-Item -ItemType Directory -Path $directory -Force | Out-Null
    $thumbs | Select-Object -Unique | Set-Content -Path $InstallerStatePath -Encoding ascii
}

function Remove-TrackedCertThumbprints {
    param([string[]]$Thumbprints = @())
    $Thumbprints = @($Thumbprints | ForEach-Object { $_.ToUpperInvariant() } | Where-Object { $_ -match '^[A-F0-9]{40}$' } | Select-Object -Unique)
    if ($Thumbprints.Count -eq 0) { return }
    $isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
    $scriptText = '$thumbs = @(' + (($Thumbprints | ForEach-Object { "'$_'" }) -join ',') + '); Get-ChildItem Cert:\LocalMachine\TrustedPeople -ErrorAction SilentlyContinue | Where-Object { $thumbs -contains $_.Thumbprint.ToUpperInvariant() } | Remove-Item -Force -ErrorAction Stop'
    $encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($scriptText))
    if ($isAdmin) {
        & powershell.exe -NoProfile -ExecutionPolicy Bypass -EncodedCommand $encoded
        if ($LASTEXITCODE -ne 0) { throw "Could not remove installer-managed certificates (exit code $LASTEXITCODE)." }
    }
    else {
        $proc = Start-Process -FilePath (Join-Path $env:SystemRoot "System32\WindowsPowerShell\v1.0\powershell.exe") -Verb RunAs -Wait -PassThru -ArgumentList @("-NoProfile", "-ExecutionPolicy", "Bypass", "-EncodedCommand", $encoded)
        if ($proc.ExitCode -ne 0) { throw "Administrator approval did not remove installer-managed certificates (exit code $($proc.ExitCode))." }
    }
}

function Invoke-MsixSignatureVerification {
    param(
        [Parameter(Mandatory)][string]$SignToolPath,
        [Parameter(Mandatory)][string]$PackagePath
    )
    $logStem = Join-Path $env:TEMP ("netflow_signtool_" + [guid]::NewGuid().ToString("N"))
    $stdoutPath = "$logStem.out"
    $stderrPath = "$logStem.err"
    try {
        & $SignToolPath verify /pa /v $PackagePath 1> $stdoutPath 2> $stderrPath
        $exitCode = $LASTEXITCODE
        $stdoutText = if (Test-Path $stdoutPath) { Get-Content -LiteralPath $stdoutPath -Raw } else { "" }
        $stderrText = if (Test-Path $stderrPath) { Get-Content -LiteralPath $stderrPath -Raw } else { "" }
        return [pscustomobject]@{
            ExitCode = $exitCode
            StandardOutput = $stdoutText
            StandardError = $stderrText
            Text = ($stdoutText + [Environment]::NewLine + $stderrText).Trim()
        }
    }
    finally {
        Remove-Item -LiteralPath $stdoutPath, $stderrPath -Force -ErrorAction SilentlyContinue
    }
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

    return $null
}

function Install-SideloadCert {
    param([System.Security.Cryptography.X509Certificates.X509Certificate2]$Certificate)

    if (-not $Certificate) { throw "The package signing certificate is missing." }
    $thumb = $Certificate.Thumbprint

    # Device trust for MSIX package sideloading must be in LocalMachine\TrustedPeople
    $inLocalTrusted = Get-ChildItem Cert:\LocalMachine\TrustedPeople -ErrorAction SilentlyContinue |
    Where-Object { $_.Thumbprint -eq $thumb }

    if (-not $inLocalTrusted) {
        $tempCer = Join-Path $env:TEMP ("netflow_cert_" + $thumb + ".cer")
        [System.IO.File]::WriteAllBytes($tempCer, $Certificate.Export([System.Security.Cryptography.X509Certificates.X509ContentType]::Cert))

        $imported = $false
        try {
            Import-Certificate -CertStoreLocation "Cert:\LocalMachine\TrustedPeople" -FilePath $tempCer -ErrorAction Stop | Out-Null
            $imported = $true
        }
        catch {
            Write-Host "Prompting for administrator approval to trust certificate..." -ForegroundColor Cyan
            $path64 = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($tempCer))
            $command = '$p = [Text.Encoding]::Unicode.GetString([Convert]::FromBase64String("' + $path64 + '")); Import-Certificate -CertStoreLocation Cert:\LocalMachine\TrustedPeople -FilePath $p -ErrorAction Stop | Out-Null'
            $encoded = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($command))
            $proc = Start-Process -FilePath (Join-Path $env:SystemRoot "System32\WindowsPowerShell\v1.0\powershell.exe") -Verb RunAs -Wait -PassThru -ArgumentList @("-NoProfile", "-ExecutionPolicy", "Bypass", "-EncodedCommand", $encoded)
            if ($proc.ExitCode -ne 0) { throw "Administrator approval failed to trust the package certificate (exit code $($proc.ExitCode))." }
            $imported = $true
        }
        finally {
            Remove-Item $tempCer -Force -ErrorAction SilentlyContinue
        }
        $verified = Get-ChildItem Cert:\LocalMachine\TrustedPeople -ErrorAction SilentlyContinue | Where-Object { $_.Thumbprint -eq $thumb }
        if (-not $verified) { throw "The package certificate was not added to LocalMachine\TrustedPeople." }
        Write-Host "Package signing certificate is trusted in LocalMachine\TrustedPeople." -ForegroundColor Green
    }
    else {
        Write-Host "Package signing certificate is trusted in LocalMachine\TrustedPeople." -ForegroundColor Green
    }
    return [bool]$imported
}

function New-MsixPackage {
    $tools = Find-SdkTools
    if (-not $tools.MakeAppx -or -not $tools.SignTool) {
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

    # Read and adapt the manifest to the architecture of the executable being packaged.
    $manifestXml = Read-XmlFile -Path $manifest
    $identityNode = $manifestXml.SelectSingleNode("//*[local-name()='Identity']")
    if (-not $identityNode) { throw "Package manifest has no Identity element: $manifest" }
    $architecture = Get-PeArchitecture -Path $binExe
    $identityNode.SetAttribute("ProcessorArchitecture", $architecture)
    $manifestContent = $manifestXml.OuterXml
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
            -FriendlyName "Net Flow Local Sideload Signing" `
            -CertStoreLocation "Cert:\CurrentUser\My" `
            -TextExtension @("2.5.29.37={text}1.3.6.1.5.5.7.3.3", "2.5.29.19={text}")
    }
    else {
        Write-Host "Reusing active local signing certificate (Expires: $($cert.NotAfter.ToShortDateString()))." -ForegroundColor Green
    }

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
        }
        elseif ($tools.MakePri) {
            $priConfig = Join-Path $tempLayout "priconfig.xml"
            & $tools.MakePri createconfig /cf $priConfig /dq "en-US" /pv "10.0.0" /o | Out-Null
            if ($LASTEXITCODE -ne 0) { throw "makepri.exe createconfig failed with exit code $LASTEXITCODE" }
            & $tools.MakePri new /pr $tempLayout /cf $priConfig /of (Join-Path $tempLayout "resources.pri") /o | Out-Null
            if ($LASTEXITCODE -ne 0) { throw "makepri.exe new failed with exit code $LASTEXITCODE" }
            Remove-Item $priConfig -Force -ErrorAction SilentlyContinue
        }
        else { throw "resources.pri and makepri.exe are both unavailable; cannot build a complete package." }

        # Pack MSIX
        $outputMsix = Join-Path $env:TEMP ("NetFlow_" + [guid]::NewGuid().ToString("N") + ".msix")

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
        $script:GeneratedMsixPath = (Resolve-Path $outputMsix).Path
        $script:GeneratedSigningCertificate = $cert
        return $outputMsix
    }
    finally {
        if (Test-Path $tempLayout) {
            Remove-Item $tempLayout -Recurse -Force -ErrorAction SilentlyContinue
        }
    }
}

function Get-Status {
    Show-Header
    Write-Host "Location        : $ScriptDir"

    $identityName = Get-PackageIdentity
    $appx = Get-AppxPackage -Name $identityName -ErrorAction SilentlyContinue
    $msix = Find-ExistingMsix
    if ($msix) {
        $msixSize = [math]::Round(((Get-Item $msix).Length / 1MB), 2)
        Write-Host "MSIX Package    : [FOUND] $msix ($msixSize MB)" -ForegroundColor Green
    }
    elseif ($appx) {
        Write-Host "MSIX Package    : [NO LOCAL FILE] (installed package is registered with Windows)" -ForegroundColor Gray
    }
    else {
        Write-Host "MSIX Package    : [NOT FOUND]" -ForegroundColor Yellow
    }

    $bin = Find-ExistingBinary
    Write-Host "Executable Found: $(if ($bin) { '[YES] ' + $bin } else { '[NO]' })" -ForegroundColor $(if ($bin) { 'Green' } else { 'Gray' })

    $manifest = Get-PackageManifestPath
    Write-Host "Manifest Found  : $(if ($manifest) { '[YES] ' + $manifest } else { '[NO]' })" -ForegroundColor $(if ($manifest) { 'Green' } else { 'Gray' })

    if ($appx) {
        Write-Host "Widget Package  : [INSTALLED] $($appx.PackageFullName)" -ForegroundColor Green
        Write-Host "Install Location: $($appx.InstallLocation)" -ForegroundColor Gray
    }
    else {
        Write-Host "Widget Package  : [NOT INSTALLED] (Identity: $identityName)" -ForegroundColor Yellow
    }

    $runningProc = Get-Process -Name "net-flow" -ErrorAction SilentlyContinue
    Write-Host "Widget Process  : $(if ($runningProc) { '[RUNNING] (PID: ' + $runningProc.Id + ')' } else { '[IDLE]' })" -ForegroundColor $(if ($runningProc) { 'Green' } else { 'Gray' })
    Write-Host ""
}

function Install-Package {
    Show-Header

    $msixToInstall = $null

    if ($MsixPath) {
        if (-not (Test-Path -LiteralPath $MsixPath -PathType Leaf)) { throw "The explicitly specified MSIX does not exist: $MsixPath" }
        Write-Host "Using explicitly specified MSIX: $MsixPath" -ForegroundColor Green
        $msixToInstall = (Resolve-Path $MsixPath).Path
    }
    elseif (-not $Rebuild) {
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

    $metadata = Get-MsixMetadata -Path $msixToInstall
    $expectedIdentity = Get-PackageIdentity
    $expectedPublisher = Get-PackagePublisher
    if ($metadata.Name -ne $expectedIdentity) { throw "Package identity mismatch. Expected '$expectedIdentity', found '$($metadata.Name)'." }
    if ($metadata.Publisher -ne $expectedPublisher) { throw "Package publisher mismatch. Expected '$expectedPublisher', found '$($metadata.Publisher)'." }

    # Reject unsigned, damaged, expired, or unexpected packages before trusting any signer.
    $sig = Get-AuthenticodeSignature -FilePath $msixToInstall -ErrorAction Stop
    $signatureStatus = [string]$sig.Status
    $signingCertificate = $sig.SignerCertificate
    if (-not $signingCertificate -and $script:GeneratedSigningCertificate) {
        $signingCertificate = $script:GeneratedSigningCertificate
    }
    if ($signatureStatus -notin @("Valid", "NotTrusted", "UnknownError") -or -not $signingCertificate) {
        throw "Package signature validation failed (status: $($sig.Status))."
    }
    if ($signingCertificate.Subject -ne $metadata.Publisher) { throw "Package signer does not match its manifest publisher." }
    $now = Get-Date
    if ($signingCertificate.NotBefore -gt $now -or $signingCertificate.NotAfter -lt $now) { throw "Package signing certificate is outside its validity period." }
    $sdkTools = Find-SdkTools
    if (-not $sdkTools.SignTool) { throw "signtool.exe is required to verify an MSIX package. Install the Windows SDK or add signtool.exe to PATH." }
    if ($signatureStatus -eq "UnknownError") {
        Write-Host "Package signer: $($signingCertificate.Subject) (PowerShell cannot classify MSIX signature; checking with SignTool)" -ForegroundColor Cyan
    }
    else {
        Write-Host "Package signer: $($signingCertificate.Subject) (PowerShell status: $signatureStatus)" -ForegroundColor Cyan
    }

    # Verify package integrity before adding machine trust. A first-use self-signed
    # certificate may produce exactly one SignTool chain-trust error; no other error
    # is safe to waive before the certificate is trusted.
    $preverification = Invoke-MsixSignatureVerification -SignToolPath $sdkTools.SignTool -PackagePath $msixToInstall
    $preverifyExitCode = $preverification.ExitCode
    if ($preverifyExitCode -ne 0) {
        $preverifyText = $preverification.Text
        $normalizedPreverifyText = [regex]::Replace($preverifyText, '\s+', ' ')
        $expectedThumbprint = $signingCertificate.Thumbprint.Replace(" ", "").ToUpperInvariant()
        $hasExpectedSigner = $preverifyText.Replace(" ", "").ToUpperInvariant().Contains($expectedThumbprint)
        $isOnlyUntrustedChain = $normalizedPreverifyText -match '(?i)SignTool Error: A certificate chain processed, but terminated in a root certificate which is not trusted by the trust provider' -and
            $normalizedPreverifyText -match '(?i)Number of errors: 1\b'
        if (-not ($hasExpectedSigner -and $isOnlyUntrustedChain)) {
            throw "SignTool could not verify the package before certificate trust was added (exit code $preverifyExitCode):`n$preverifyText"
        }
    }

    $newlyTrusted = Install-SideloadCert -Certificate $signingCertificate
    $verification = Invoke-MsixSignatureVerification -SignToolPath $sdkTools.SignTool -PackagePath $msixToInstall
    $verifyExitCode = $verification.ExitCode
    if ($verifyExitCode -ne 0) {
        if ($newlyTrusted) { Remove-TrackedCertThumbprints -Thumbprints @($signingCertificate.Thumbprint) }
        throw "SignTool could not verify the MSIX package (exit code $verifyExitCode):`n$($verification.Text)"
    }
    Write-Host "MSIX signature verified by SignTool." -ForegroundColor Green
    if ($newlyTrusted) { Save-TrackedCertThumbprint -Thumbprint $signingCertificate.Thumbprint }

    Write-Host "Installing Net Flow MSIX package: $msixToInstall" -ForegroundColor Cyan

    # Stop any active Net Flow instance
    Write-Host "Stopping any running Net Flow processes..." -ForegroundColor Cyan
    Stop-Process -Name "net-flow" -Force -ErrorAction SilentlyContinue
    Start-Sleep -Milliseconds 300

    # Add-AppxPackage performs an in-place package update when the identity matches.
    Write-Host "Installing Net Flow package into Windows..." -ForegroundColor Cyan
    try {
        Add-AppxPackage -Path $msixToInstall -ForceApplicationShutdown
        Write-Host "Net Flow package installed successfully!" -ForegroundColor Green
        if ($script:GeneratedMsixPath -and (Test-Path -LiteralPath $script:GeneratedMsixPath)) {
            Remove-Item -LiteralPath $script:GeneratedMsixPath -Force -ErrorAction Stop
            Write-Host "Removed the MSIX generated for this installation." -ForegroundColor Gray
            $script:GeneratedMsixPath = $null
        }
    }
    catch {
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
    $thumbprints = @(Get-TrackedCertThumbprints)
    if ($thumbprints.Count -eq 0) {
        Write-Host "No installer-managed machine certificates are recorded; leaving other certificates untouched." -ForegroundColor Gray
        return
    }
    Write-Host "Removing installer-managed signing certificates..." -ForegroundColor Yellow
    Remove-TrackedCertThumbprints -Thumbprints $thumbprints
    Remove-Item -LiteralPath $InstallerStatePath -Force -ErrorAction SilentlyContinue
    Write-Host "Installer-managed signing certificates removed." -ForegroundColor Green
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
        Write-Host "Net Flow widget package uninstalled successfully." -ForegroundColor Green
    }
    else {
        Write-Host "Package '$identityName' is not currently installed." -ForegroundColor Yellow
    }

    Remove-SideloadCert

    if ($PurgeData) {
        $appDataNetFlow = Join-Path $env:LOCALAPPDATA "NetFlow"
        if (Test-Path $appDataNetFlow) {
            Write-Host "Purging application data and history ($appDataNetFlow)..." -ForegroundColor Yellow
            Remove-Item -LiteralPath $appDataNetFlow -Recurse -Force -ErrorAction Stop
            Write-Host "Net Flow application data purged." -ForegroundColor Green
        }
    }
    else { Write-Host "Application data was preserved. Use -PurgeData with -Uninstall to remove it." -ForegroundColor Gray }

    Restart-WidgetBoard

    Write-Host ""
    Write-Host "Net Flow uninstallation complete." -ForegroundColor Green
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
if ($PurgeData -and -not $Uninstall) { throw "-PurgeData can only be used together with -Uninstall." }
if ($MsixPath -and $Rebuild) { throw "Use either -MsixPath or -Rebuild; the options are mutually exclusive." }
$exclusiveModes = @($Uninstall, $RemoveCert, $Status, $RestartWidgets) | Where-Object { $_ }
if ($exclusiveModes.Count -gt 1) { throw "Choose only one of -Uninstall, -RemoveCert, -Status, or -RestartWidgets." }
if ($exclusiveModes.Count -gt 0 -and ($Install -or $Rebuild -or $MsixPath)) { throw "Install options cannot be combined with status, uninstall, certificate removal, or widget restart." }

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
elseif ($Install -or $Rebuild -or $MsixPath) {
    Install-Package
    Write-Host ""
    Get-Status
}
else {
    # Direct running without explicit flags
    $identityName = Get-PackageIdentity
    $appx = Get-AppxPackage -Name $identityName -ErrorAction SilentlyContinue

    if ($appx) {
        Show-Header
        Write-Host "Net Flow is currently installed: $($appx.PackageFullName)" -ForegroundColor Cyan
        Write-Host ""
        Write-Host "What would you like to do?" -ForegroundColor Yellow
        Write-Host "  [U] Uninstall Net Flow (remove package and installer-managed certificates; keep data)"
        Write-Host "  [R] Reinstall / Update Net Flow"
        Write-Host "  [C] Cancel / Exit"
        Write-Host ""
        $choice = Read-Host "Select an option [U/R/C] (Default: C)"
        if ($choice.Trim().ToUpper() -eq "U") {
            Uninstall-Package
        }
        elseif ($choice.Trim().ToUpper() -eq "R") {
            Install-Package
            Write-Host ""
            Get-Status
        }
        else {
            Write-Host "Operation cancelled." -ForegroundColor Gray
        }
    }
    else {
        # Not currently installed, proceed with default installation
        Install-Package
        Write-Host ""
        Get-Status
    }
}
