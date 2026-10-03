<div align="center" markdown="1">

<img src="widget/Assets/MasterLogo.png" alt="Net Flow Logo" width="128" height="128" style="border-radius: 28px;" />

# 📊 Net Flow

**Real-Time Network Bandwidth Telemetry & Native Windows 11 Widget**

_Built in pure Rust for maximum performance, buttery-smooth fluid waveforms, and near-zero resource footprint._

[![CI](https://img.shields.io/badge/CI-Passing-brightgreen?logo=github-actions&logoColor=white)](https://github.com/rockerrishabh/net-flow/actions/workflows/ci.yml)
[![Version](https://img.shields.io/badge/Version-0.9.0-blue?logo=windows&logoColor=white)](CHANGELOG.md)
[![Microsoft Store](https://img.shields.io/badge/Microsoft%20Store-9PCR54NGJ94J-0078D4?logo=microsoftstore&logoColor=white)](https://apps.microsoft.com/detail/9PCR54NGJ94J?mode=direct&cid=github_shield)
[![Platform](https://img.shields.io/badge/Platform-Windows%2011%20(x64%20%2F%20ARM64)-0078D4?logo=windows11&logoColor=white)](https://www.microsoft.com/windows)
[![Rust](https://img.shields.io/badge/Language-Rust%202024-DEA584?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](#-license)

<br/>
<br/>

<a href="https://apps.microsoft.com/detail/9PCR54NGJ94J?mode=direct&cid=github_hero">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://get.microsoft.com/images/en-us%20dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://get.microsoft.com/images/en-us%20light.svg">
    <img src="https://get.microsoft.com/images/en-us%20dark.svg" alt="Get Net Flow from Microsoft Store" width="220" />
  </picture>
</a>

</div>

---

## ✨ Highlights

- **📋 Diagnostic Telemetry & Export Engine**: Generate comprehensive, reproducible network diagnostic reports via canonical **JSON** (`schema_version = 1`) and the specialized **CSV Dialect** (`netflow-diagnostics-v1`). Exports multi-adapter IP/MAC/gateway/DNS configuration, deep Wi-Fi RF telemetry, dual-stack ICMP quality with exact mean absolute RTT difference jitter, active process socket attribution, and a continuous 90-day zero-filled historical daily usage series.
- **🖥️ Headless CLI & Stdout Automation**: Headless document extraction via `net-flow.exe --export <csv|json> [path]`. Piping to stdout (`-`) guarantees pristine data streams with all error logs and diagnostics directed strictly to stderr for seamless pipe composition (`net-flow.exe --export json - | jq .`).
- **🪟 Native Win32 Tray Flyout Companion (<5 MB RAM)**: Ultra-lightweight popup companion summoned instantly by left-clicking the notification area icon (`WM_LBUTTONUP`). Powered by pure GDI double-buffering with zero XAML/WinUI runtime overhead, displaying live metric cards, sparklines, latency diagnostics, quota gauges, and a balanced 3-button action bar (`[Export]`, `[Reset]`, `[Close]`).
- **🦾 Windows on ARM64 Tier-1 Support**: Native binary compilation for `aarch64-pc-windows-msvc` alongside `x86_64`, delivering fluid performance and extreme battery efficiency on Snapdragon X Elite and Copilot+ PCs.

- **🪟 Native Windows 11 Widgets Board Integration**: First-class widget integration (<kbd>Win</kbd> + <kbd>W</kbd>) with native Adaptive Cards v1.6 support across **Small**, **Medium**, and **Large** card dimensions, powered by modern **Windows App SDK 2.x**.
- **💳 Data Budgeting, Quotas & Rolling History**: User-configurable monthly data cap allowance (GB), custom renewal days with automatic month-end and leap-year clamping, multi-tier crossing warning toasts (80%, 90%, 100%), and 90-day bounded atomic JSON history.
- **⚡ Dual-Stack IPv6 & IPv4 Latency Engine**: True dual-stack ICMP telemetry with automatic IPv6 preference and seamless IPv4 fallback using native Win32 `Icmp6SendEcho2` and `IcmpSendEcho2`, probing gateway or public internet with ping jitter spread (`18 ms · ±2 ms`).
- **📉 20-Sample Rolling Packet Loss & Semantic Health**: Continuously calculates packet loss % across an authoritative circular window (~40s), tracking clear semantic health states (`Healthy`, `Degraded`, `Timeout`, `Unavailable`).
- **🛜 Deep Wi-Fi PHY & Physical Link Telemetry**: Real-time physical layer metrics using Windows Native Wi-Fi API (`WLAN_REALTIME_CONNECTION_QUALITY`) without requiring location permissions: detects Wi-Fi 7 / 6 / 5 generations, frequency bands (2.4 / 5 / 6 GHz), RSSI dBm, negotiated rates, Multi-Link Operation (MLO), and Ethernet link speeds.
- **🛡️ Persistent Background System Tray Host**: Decoupled notification-area host (`net-flow.exe --tray`) started automatically at login via Windows `<uap5:StartupTask>`, featuring quick session resets, startup toggling, and fast latency target cycling.
- **📈 Mirrored Dual-Stream Waveforms**: An in-memory supersampled sparkline rendering download traffic above the baseline and upload traffic below on a shared scale with glowing pulse nodes and peak-preserving Catmull-Rom smoothing.
- **🌓 System Theme-Aware Palettes**: Automatic Windows Dark/Light mode tracking via `AppsUseLightTheme` registry integration, rendering vibrant dark palettes or high-contrast light palettes with in-card overrides.
- **📊 Modern Graph Rendering Styles**: Choose between smooth **Area** (Catmull-Rom spline fills) or discrete industrial **Bar** (quantized histogram columns).
- **🚀 Instant Auto-Start on Boot**: Native packaged Win32 `<uap5:StartupTask>` launches the persistent host upon Windows login with zero configuration.
- **🌊 50 KB/s Scale Floor & Headroom**: A vertical scale floor keeps sub-kilobyte background network noise proportional, while 18% vertical headroom cushions traffic spikes from card boundaries.
- **⚡ Ultra-Low Resource Usage**: Uses native Windows `IP Helper` (`GetIfTable2`) APIs and asynchronous Rust for near-zero CPU (< 0.1%) and negligible RAM footprint (< 15 MB).
- **🛜 Smart Active Adapter Detection**: Auto-detects the primary active network adapter with contextual badges (Wi-Fi with friendly SSID, Ethernet, Cellular, VPN).
- **📱 Per-App Bandwidth Attribution**: Tracks active applications consuming network bandwidth with compact rate formatting (`↓ 11.1 ↑ 9.1 KB/s (37)`) and generous 22-character name budgets.
- **📊 Authoritative Session Usage Tracking**: Monitors cumulative upload/download data transferred and active session duration with atomic disk persistence and race-free multi-process synchronization.
- **🔔 Asymmetric Bandwidth Usage Alerts**: Independent download and upload state machines fire Windows toasts only when sustained above independent thresholds, with rate-limiting cooldown timers.
- **🖥️ System Tray Icon**: A rich notification-area icon with live transfer rate and latency tooltips, widgets board launcher, startup toggles, and session reset.
- **⚙️ Native Widget Customization**: Seamlessly customize speed units, timeframe windows, theme modes, graph styles, and alert thresholds via Windows 11 Widget Board's native flyout.

---

## 🪟 Native Win32 Tray Flyout Companion

Net Flow v0.8.0 introduces an ultra-lightweight companion flyout summoned instantly by **left-clicking the system tray icon**:

- **⚡ Near-Zero Overhead (<4 MB RAM)**: Pure Win32 GDI double-buffered rendering (`CreateCompatibleDC` + `CreateCompatibleBitmap`) blitted on `WM_PAINT`. Operates without loading XAML, WinUI, or DirectComposition runtimes.
- **📍 Precision Taskbar Positioning**: Queries `Shell_NotifyIconGetRect` for exact notification icon coordinates, dynamically calculates orientation across bottom, top, left, or right taskbars, and clamps against multi-monitor work areas (`GetMonitorInfoW`).
- **🎯 Non-Blocking Presentation Isolation**: The UI thread strictly renders from an immutable `FlyoutSnapshot` prepared by the background telemetry worker—never blocking on network sockets, WLAN, or IP Helper queries.
- **🎨 Windows 11 Chrome & Per-Monitor DPI**: Automatically adapts to Windows dark/light mode (`DWMWA_USE_IMMERSIVE_DARK_MODE`) with rounded corners (`DWMWCP_ROUND`). Dynamically resizes typography and backbuffers on `WM_DPICHANGED`.
- **📊 Live Dashboard Components**:
  - **Metric Cards**: Prominent download (emerald) and upload (blue) current rate displays.
  - **Latency Diagnostics**: Real-time ping latency, jitter spread, packet loss %, and semantic health status pill.
  - **Dual Sparkline Waveform**: 30-sample rolling timeline visualizing download and upload rails.
  - **Data Quota Gauge**: Color-coded progress bar tracking current monthly allowance and billing cycle.
  - **Top Network Consumers**: Live process attribution displaying the top 3 active bandwidth-consuming applications.
  - **Quick Controls**: Interactive "Reset Session" and "Close" buttons with mouse hover tracking and click hit-testing.

---

## 📥 Installation

### 1. Microsoft Store (Recommended)

Net Flow is available directly through the Microsoft Store with seamless background updates for both **x64** and **ARM64** devices:

<a href="https://apps.microsoft.com/detail/9PCR54NGJ94J?mode=direct&cid=github_install">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://get.microsoft.com/images/en-us%20dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://get.microsoft.com/images/en-us%20light.svg">
    <img src="https://get.microsoft.com/images/en-us%20dark.svg" alt="Download Net Flow from Microsoft Store" width="200" />
  </picture>
</a>

👉 _Or launch directly in the Windows Store app via protocol:_ [`ms-windows-store://pdp/?productid=9PCR54NGJ94J`](ms-windows-store://pdp/?productid=9PCR54NGJ94J)

### 2. Sideload Pre-built Release Archive

Download the matching archive for your CPU architecture from [GitHub Releases](https://github.com/rockerrishabh/Net-Flow/releases/latest):

| Architecture | Package Archive | Target Devices |
| :--- | :--- | :--- |
| **x64 (Intel / AMD)** | `net-flow-windows-x64.zip` | 64-bit Windows 11 PCs |
| **ARM64 (Qualcomm / Snapdragon)** | `net-flow-windows-arm64.zip` | Snapdragon X Elite / Copilot+ PCs |

1. Extract the downloaded zip archive.
2. Run `.\install.ps1` in PowerShell (or right-click `install.ps1` → **Run with PowerShell**).
3. The installer trusts the package signing certificate in **Local Computer → Trusted People**, registers the MSIX package into Windows 11, and refreshes the Widgets Board.

### 3. Developer Source Build & Sideloading

If building from source or testing local modifications:

```powershell
# Clone the repository
git clone https://github.com/rockerrishabh/net-flow.git
cd net-flow

# Fast install (uses existing built package if available)
powershell -ExecutionPolicy Bypass -File scripts/install.ps1

# Force fresh recompilation and repackaging from source
powershell -ExecutionPolicy Bypass -File scripts/install.ps1 -Rebuild
```

#### Installer Options (`scripts/install.ps1`)

| Parameter | Description |
| :--- | :--- |
| *(default)* | Installs existing valid MSIX package if present; packages if missing. |
| `-Rebuild` | Forces clean Cargo compilation (`cargo build --release`), repacks MSIX, and signs with a local certificate. |
| `-MsixPath <path>` | Installs an explicitly specified external `.msix` package. |
| `-Status` | Displays current installation, binary, manifest, and background process status. |
| `-Uninstall` | Safely unregisters the widget package (`NetFlow.Widget`) and sweeps dev certificates. |
| `-RestartWidgets` | Best-effort refresh of Windows 11 Widgets Board host processes. |

> [!NOTE]
> Local MSIX sideloading requires **Developer Mode** enabled in Windows Settings (*Settings → System → For developers → Developer Mode*) and trusts the self-signed package in `Local Computer\TrustedPeople`.

---

## ⚡ Performance & Architectural Hardening

Net Flow is engineered from the ground up for zero distraction, extreme reliability, and minimal system impact:

| Metric / Component | Implementation | Impact |
| :--- | :--- | :--- |
| **Release Binary Size** | Link-Time Optimization (`lto = true`, `strip = true`, `codegen-units = 1`) | **Compact standalone executable** (LTO stripped) |
| **Tray Flyout Working Set** | Pure Win32 GDI double-buffering (zero XAML/WinUI runtime overhead) | **< 4 MB RAM** companion window |
| **Platform Target Support** | Dual native Tier-1 targets (`x86_64-pc-windows-msvc` & `aarch64-pc-windows-msvc`) | **Native x64 and ARM64** execution |
| **Active CPU Utilization** | Direct Win32 IP Helper polling (`GetIfTable2`) & diffing | **< 0.1% CPU** during active monitoring |
| **Idle CPU Overhead** | Precomputed `OnceLock` idle chart cache bypassing rasterizer | **0.024% of 1 core** (55x speedup; 120 µs idle bypass) |
| **Memory Footprint** | Bounded ring buffers (240 samples) & in-memory rasterization | **< 15 MB** working set |
| **Telemetry Cadence** | Decoupled 250ms (4 Hz) sampling + 500ms (2 Hz) card publication | High-resolution waveforms with zero system scheduler jitter |
| **Process Attribution** | Decoupled 1.0s process inspection with 256-entry bounded icon cache | Per-app network tracking without UI thread blocking |
| **Chart Peak Tracking** | Incremental O(1) running maximum tracking | Eliminates O(N) buffer scans on every render tick |
| **COM Lifetime** | Automatic idle detection with 30s grace period and `CoRevokeClassObject` | **Zero zombie background processes** when unpinned |
| **Lock Poison-Safety** | Poison-recovering extension traits (`lock_safe`, `read_safe`, `write_safe`) | Fault-tolerant under `panic = "abort"` |
| **Log Management** | Thread-safe 1MB rotating logger in `%TEMP%` | Prevents disk bloat; quiet by default |

---

## 🔒 Privacy & Permissions (Why "Precise Location"?)

When running Net Flow, Windows 11 may show Net Flow under **Settings → Privacy & security → Location** or briefly show the taskbar location icon.

### Why Does Windows Flag This?

1. **Friendly Wi-Fi Name**: In the card header, Net Flow displays the name of your active Wi-Fi network (e.g. `Home-5G` instead of generic `Wi-Fi`).
2. **The Win32 Wi-Fi API**: To read that SSID, Net Flow queries the Windows Native Wi-Fi API (`wlanapi.dll` via `WlanQueryInterface`).
3. **Microsoft's Privacy Grouping**: Because nearby Wi-Fi network names (SSIDs and BSSIDs) can theoretically be cross-referenced against global Wi-Fi positioning databases to estimate a device's physical location, Windows 10/11 classifies all native Wi-Fi scanning and query APIs under the **"Precise Location"** permission toggle.

### Our Privacy Guarantee

- **Zero Location Tracking**: Net Flow contains **zero GPS code, zero geolocation libraries, and makes zero network requests**.
- **100% Offline**: Net Flow does not transmit any data over the internet. There are **zero diagnostics, zero analytics, and zero telemetry servers**.
- **Completely Optional**: If you disable location in Windows Settings, Net Flow continues running with 100% functionality—it simply displays `Wi-Fi` in the header instead of your network name.

📖 For complete details, see our [Privacy Policy (PRIVACY.md)](PRIVACY.md).

---

## 🏗️ Architecture

Net Flow is structured as a modular, high-performance Rust workspace:

```text
net-flow/
├── .github/
│   └── workflows/
│       ├── ci.yml                     # Continuous integration & dual-architecture validation
│       └── release.yml                # Unified GitHub Release & Microsoft Store bundle publishing
├── crates/
│   └── core/                          # net-flow-core (pure Rust core logic)
│       ├── src/
│       │   ├── alerts.rs              # Sustained-threshold bandwidth alert engine & config
│       │   ├── backend.rs             # IP Helper polling, per-adapter tracking & classification
│       │   ├── card.rs                # Adaptive Cards v1.6 JSON builders & templates
│       │   ├── chart.rs               # In-memory dual-stream waveform rasteriser
│       │   ├── format.rs              # Bandwidth scaling & humanized unit formatting
│       │   ├── icons.rs               # Embedded vector glyphs (PNG data URIs)
│       │   ├── lib.rs                 # Compile-time constants, ARCH target detection & module exports
│       │   └── process.rs             # Active process network attribution & bounded icon cache
├── widget/                            # net-flow (Windows App SDK COM widget provider & Win32 tray host)
│   ├── Assets/                        # Master branding, high-DPI logos, app.ico, and favicon pack
│   │   ├── MasterLogo.png             # 816x816 high-res squircle master logo
│   │   ├── Square150x150Logo.png      # 300x300 Windows tile logo
│   │   ├── Square44x44Logo.png        # 88x88 widget header & provider logo
│   │   ├── StoreLogo.png              # 100x100 Microsoft Store logo
│   │   ├── app.ico                    # Multi-res Windows executable icon (256, 128, 64, 48, 32, 16)
│   │   └── favicon.ico                # Web & docs favicon suite (48, 32, 16)
│   ├── Package.appxmanifest           # Open-source generic manifest template
│   ├── build.rs                       # Resource compilation & icon embedding
│   └── src/
│       ├── bindings.rs                # Windows App SDK WinMD bindings
│       ├── factory.rs                 # Out-of-proc COM ClassFactory implementation
│       ├── flyout.rs                  # Native Win32 companion flyout, GDI double-buffering & DPI scaling
│       ├── main.rs                    # WinMain entry point, COM lifecycle & idle shutdown
│       ├── provider.rs                # IWidgetProvider2 handler with poison-resilient locks
│       ├── toast.rs                   # Packaged-app bandwidth alert toast delivery
│       └── tray.rs                    # Notification-area tray icon, tooltip, context menu & left-click flyout trigger
├── scripts/
│   └── install.ps1                    # Sideload packaging, certificate provisioning & registration
├── Cargo.toml                         # Workspace manifest & LTO release profile
├── CHANGELOG.md                       # Release notes & version history
├── CODE_OF_CONDUCT.md                 # Contributor Covenant v2.1
├── CONTRIBUTING.md                     # Contributor guide & developer workflow
├── LICENSE-APACHE                     # Apache 2.0 License
├── LICENSE-MIT                        # MIT License
├── PRIVACY.md                         # Store-ready Privacy Statement & location disclosure
└── SECURITY.md                        # Security policy & vulnerability reporting
```

---

## 🧪 Testing & Verification

Run all **157 workspace unit tests**:

```powershell
cargo test --workspace
```

Run Clippy with strict zero-warnings enforcement:

```powershell
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Build the optimized release binary:

```powershell
cargo build --release --workspace
```

---

## 📦 CI/CD & Microsoft Store Publishing

Net Flow includes automated GitHub Actions workflows:

1. **`ci.yml`**: Runs on every push and pull request across both `x86_64` and `aarch64` targets. Validates code formatting, executes all **157 automated unit tests**, and verifies MSIX layout packaging.
2. **`release.yml`**: Triggered on Git tags (e.g. `v0.9.0`) or manual workflow dispatch. Builds optimized binaries for both x64 and ARM64, packages public sideload archives (`net-flow-windows-x64.zip` and `net-flow-windows-arm64.zip`) with SHA256 checksums, bundles both architectures into a unified Microsoft Store package (`NetFlow_Store.msixbundle`), performs pre-submission integrity validation, and automatically submits/updates the package and "What's new" metadata in the **Microsoft Store** catalog via Partner Center.

---

## 🗺️ Roadmap to v1.0.0

Net Flow follows a focused, incremental path towards production-ready General Availability (v1.0.0):

| Milestone | Theme | Key Capabilities | Status |
| :--- | :--- | :--- | :---: |
| **v0.5.0** | **Modern Runtime & Polish** | Windows App SDK 2.x, ICMP jitter variance, and expanded tray host controls | ✅ Released |
| **v0.6.0** | **Network Health Expansion** | Dual-stack IPv6 ICMP, rolling packet loss % calculation, and deep Wi-Fi PHY metrics (RSSI dBm, band, link rate) | ✅ Released |
| **v0.7.0** | **Data Budgeting & History** | Monthly data cap allowances, billing cycle rollover, multi-tier toast warnings, and 90-day atomic JSON history | ✅ Released |
| **v0.8.0** | **Tray Flyout & ARM64 Hardening** | Native Win32 GDI companion flyout (<4 MB RAM), Tier-1 Windows on ARM64, and unified Store msixbundle | ✅ Released (2026-10-03) |
| **v0.9.0** | **Diagnostic Telemetry & Export Engine** | Canonical JSON & CSV export, headless CLI, multi-adapter diagnostics, exact jitter, and 3-button flyout | ✅ Released (2026-10-03) |
| **v1.0.0** | **General Availability (GA)** | Multi-language localization (i18n), zero-allocation sparklines, and 72h continuous stress hardening | 🌟 Planned |


---

## 👤 Author & Maintainer

Net Flow is an independent open-source project created and actively maintained by:

- **Rishabh Kumar**
- GitHub: [@rockerrishabh](https://github.com/rockerrishabh)
- Email: [admin@rockerrishabh.me](mailto:admin@rockerrishabh.me)

---

## 📜 Documentation

- [Changelog](CHANGELOG.md) — Version history and release notes.
- [Contributing Guide](CONTRIBUTING.md) — How to develop, test, and contribute.
- [Privacy Policy](PRIVACY.md) — Privacy commitments and location disclosure.
- [Security Policy](SECURITY.md) — Vulnerability reporting and threat model.
- [Code of Conduct](CODE_OF_CONDUCT.md) — Community standards.

---

## 📄 License

This project is dual-licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)

at your option.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Net Flow by you shall be dual-licensed as above, without any additional terms or conditions.
