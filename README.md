<div align="center" markdown="1">

<img src="widget/Assets/MasterLogo.png" alt="Net Flow Logo" width="128" height="128" style="border-radius: 28px;" />

# 📊 Net Flow

**Real-Time Network Bandwidth Telemetry & Native Windows 11 Widget**

*Built in pure Rust for maximum performance, buttery-smooth fluid waveforms, and near-zero resource footprint.*

[![CI](https://img.shields.io/badge/CI-Passing-brightgreen?logo=github-actions&logoColor=white)](https://github.com/rockerrishabh/net-flow/actions/workflows/ci.yml)
[![Version](https://img.shields.io/badge/Version-0.2.1-blue?logo=windows&logoColor=white)](CHANGELOG.md)
[![Microsoft Store](https://img.shields.io/badge/Microsoft%20Store-9PCR54NGJ94J-0078D4?logo=microsoftstore&logoColor=white)](https://apps.microsoft.com/detail/9PCR54NGJ94J?mode=direct&cid=github_shield)
[![Platform](https://img.shields.io/badge/Platform-Windows%2011-0078D4?logo=windows11&logoColor=white)](https://www.microsoft.com/windows)
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

- **🪟 Native Windows 11 Widgets Board Integration**: First-class widget integration (<kbd>Win</kbd> + <kbd>W</kbd>) with native Adaptive Cards v1.6 support across **Small**, **Medium**, and **Large** card dimensions.
- **📈 Mirrored Dual-Stream Waveforms**: An in-memory supersampled sparkline rendering download traffic above the baseline and upload traffic below on a shared scale with glowing pulse nodes.
- **🌓 System Theme-Aware Palettes**: Automatic Windows Dark/Light mode tracking via `AppsUseLightTheme` registry integration, rendering vibrant dark palettes or high-contrast light palettes with in-card overrides.
- **📊 3 Graph Rendering Styles**: Choose between smooth **Area** (Catmull-Rom spline fills), sleek **Line** (minimalist strokes), or discrete **Bar** (quantized histogram columns).
- **🚀 Instant Auto-Start on Boot**: Native packaged Win32 `<uap5:StartupTask>` launches the widget seamlessly upon Windows login, with idle grace-period termination if unpinned.
- **🌊 50 KB/s Scale Floor & Headroom**: A vertical scale floor keeps sub-kilobyte background network noise proportional, while 18% vertical headroom cushions traffic spikes from card boundaries.
- **⚡ Ultra-Low Resource Usage**: Uses native Windows `IP Helper` (`GetIfTable2`) APIs and asynchronous Rust for near-zero CPU (< 0.1%) and negligible RAM footprint (< 15 MB).
- **🛜 Smart Active Adapter Detection**: Auto-detects the primary active network adapter with contextual badges (Wi-Fi with friendly SSID, Ethernet, Cellular, VPN).
- **📱 Per-App Bandwidth Attribution**: Tracks active applications consuming network bandwidth with compact rate formatting (`↓ 11.1 ↑ 9.1 KB/s (37)`) and generous 22-character name budgets.
- **📊 Session Usage Tracking**: Monitors cumulative upload/download data transferred and active session duration with an inline **Reset** button.
- **⚙️ In-Card Customization**: Configurable speed units, timeframe windows, theme modes, and graph styles directly inside the widget card.

---

## 🚀 What's New in v0.2.0

- **🚀 Auto-Start on Windows Login**: Added native packaged Win32 `<uap5:StartupTask>` (`NetFlowStartup`) integration. Net Flow now launches cleanly on Windows user login, ensuring pinned widgets are always populated without manual intervention or resident background services.
- **🌓 Windows System Theme-Aware Sparklines**: Implemented automatic Windows Light/Dark mode detection querying `SystemUsesLightTheme` and `AppsUseLightTheme` registry keys with resilient dark fallback. The dual-stream waveform dynamically switches between Dark mode (cyan `#38D9F0` / amber `#FFB020`) and Light mode (crisp ocean `#008CB4` / warm solar `#D75F00`) high-contrast palettes, automatically matching the Windows 11 Widgets Board surface.
- **📊 Customizable Graph Styles**: Added selectable visual rendering styles: `Area` (classic Catmull-Rom filled spline), `Line` (clean minimalist strokes), and `Bar` (discrete quantized bandwidth histogram columns).
- **❄️ Theme & Style-Aware Idle Cache**: Upgraded the precomputed zero-CPU idle chart cache to index by resolved theme and graph style, guaranteeing zero rendering overhead during quiet network periods while instantaneously reflecting OS theme changes.
- **🧈 Fluid Widget Rendering & Flicker Elimination**: Architected strict separation between static visual layout templates (`SetTemplate`) and dynamic telemetry payloads (`SetData`), eliminating 500 ms full-card visual tree tearing in the Windows Widgets Board. Added collision-free payload diffing with fast 64-bit hashing and exact string equality to bypass redundant WinRT IPC updates when telemetry is static, reducing idle IPC calls by up to 96%.
- **⚙️ Enhanced In-Card Settings**: Added Theme and Graph Style ChoiceSets to the widget flip-card settings across Small, Medium, and Large form factors with full backward-compatible configuration migration.

📖 For the full historical log, check out [CHANGELOG.md](CHANGELOG.md).

---

## 📥 Installation

### 1. Microsoft Store (Recommended)
Net Flow is available directly through the Microsoft Store with seamless background updates:

<a href="https://apps.microsoft.com/detail/9PCR54NGJ94J?mode=direct&cid=github_install">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://get.microsoft.com/images/en-us%20dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://get.microsoft.com/images/en-us%20light.svg">
    <img src="https://get.microsoft.com/images/en-us%20dark.svg" alt="Download Net Flow from Microsoft Store" width="200" />
  </picture>
</a>

👉 *Or launch directly in the Windows Store app via protocol:* [`ms-windows-store://pdp/?productid=9PCR54NGJ94J`](ms-windows-store://pdp/?productid=9PCR54NGJ94J)

### 2. Local Developer Sideloading
If building from source or testing modifications locally:

```powershell
# Clone the repository
git clone https://github.com/rockerrishabh/net-flow.git
cd net-flow

# One-command build, packaging, self-signing, and sideload registration
powershell -ExecutionPolicy Bypass -File scripts/install.ps1
```

Once installed:
1. Press <kbd>Win</kbd> + <kbd>W</kbd> to open the **Windows Widgets Board**.
2. Click **+** (**Add Widgets**) in the top-right corner.
3. Select **Net Flow** and pin your preferred size (Small, Medium, or Large).

### 3. Sideload Release Bundle
1. Download `net-flow-windows-x64.zip` from [GitHub Releases](https://github.com/rockerrishabh/Net-Flow/releases).
2. Extract the archive.
3. Run `.\install.ps1` in PowerShell (or right-click `install.ps1` → **Run with PowerShell**).
The installer automatically provisions a local signing certificate, packages `NetFlow.msix`, and registers the widget package into Windows 11. To uninstall, run `.\install.ps1 -Uninstall`.

---

## ⚡ Performance & Architectural Hardening

Net Flow is engineered from the ground up for zero distraction, extreme reliability, and minimal system impact:

| Metric / Component | Implementation | Impact |
| :--- | :--- | :--- |
| **Release Binary Size** | Link-Time Optimization (`lto = true`, `strip = true`, `codegen-units = 1`) | **1.24 MB** standalone executable |
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
│       ├── ci.yml                     # Continuous integration & MSIX packaging validation
│       └── release.yml                # Unified GitHub Release & Microsoft Store publishing pipeline
├── crates/
│   └── core/                          # net-flow-core (pure Rust core logic)
│       ├── src/
│       │   ├── backend.rs             # IP Helper polling, delta computing & interface classification
│       │   ├── card.rs                # Adaptive Cards v1.6 JSON builders & templates
│       │   ├── chart.rs               # In-memory dual-stream waveform rasteriser
│       │   ├── format.rs              # Bandwidth scaling & humanized unit formatting
│       │   ├── icons.rs               # Embedded vector glyphs (PNG data URIs)
│       │   └── process.rs             # Active process network attribution & bounded icon cache
├── widget/                            # net-flow (Windows App SDK COM widget provider)
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
│       ├── main.rs                    # WinMain entry point, COM lifecycle & idle shutdown
│       └── provider.rs                # IWidgetProvider2 handler with poison-resilient locks
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

Run all 75 workspace unit tests:
```powershell
cargo test --workspace
```

Run Clippy with strict zero-warnings enforcement:
```powershell
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Build the optimized release binary (1.24 MB):
```powershell
cargo build --release --workspace
```

---

## 📦 CI/CD & Microsoft Store Publishing

Net Flow includes automated GitHub Actions workflows:

1. **`ci.yml`**: Runs on every push and pull request. Validates formatting, executes all 75 unit tests, and verifies MSIX layout packaging.
2. **`release.yml`**: Triggered on Git tags (e.g. `v0.1.1`) or manual workflow dispatch. Builds the optimized binary, packages both public sideload MSIX and Microsoft Store MSIX, generates SHA256 checksums, extracts sanitized release notes from `CHANGELOG.md`, publishes the **GitHub Release**, and automatically submits/updates the package and "What's new" metadata in the **Microsoft Store** via Partner Center.

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
