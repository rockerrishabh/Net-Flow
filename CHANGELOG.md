# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.1.0] - 2026-09-16

### Added
- **Native Windows 11 Widgets Board Integration**:
  - Full support for Small, Medium, and Large card sizes using Adaptive Cards v1.6.
  - Interactive in-card settings for speed units (Auto, B/s, KB/s, MB/s, Gbps) and timeframes (15s, 30s, 60s, 120s).
- **Dual-Stream Telemetry Sparkline**:
  - In-memory rasteriser rendering a mirrored sparkline on a shared scale (cyan download above baseline, amber upload below).
  - Anti-aliased area gradient fills and glowing live pulse indicator nodes.
  - Minimum 50 KB/s scale floor (`MIN_CHART_SCALE_BPS`) to keep sub-kilobyte background network noise proportional.
  - 18% vertical cushion/headroom preventing peak traffic spikes from touching card boundaries.
- **Active App Bandwidth Attribution**:
  - Per-process network throughput monitoring combining socket tracking and I/O heuristics.
  - Compact right-column rate notation with shared bandwidth units (`↓ 11.1 ↑ 9.1 KB/s (37)`).
  - Increased app name budget to 22 characters on Medium cards to eliminate truncation.
- **Smart Interface Detection**:
  - Auto-detection of primary active network interface (Wi-Fi, Ethernet, Cellular, VPN).
  - Safe Wi-Fi SSID resolution via Windows Native Wi-Fi API (`wlanapi.dll`).
- **Session Telemetry**:
  - Cumulative upload/download session counters and elapsed duration tracking.
  - Inline **Reset** button directly on the card to reset session statistics.
- **Brand Identity & Favicon Suite**:
  - Windows 11 Fluent dark glass acrylic squircle master logo.
  - High-DPI package assets (`Square150x150Logo.png`, `Square44x44Logo.png`, `StoreLogo.png`).
  - Multi-resolution Windows executable icon (`app.ico` embedding 256, 128, 64, 48, 32, 16 px).
  - Complete web favicon suite (`favicon.ico`, `favicon-32x32.png`, `favicon-16x16.png`, `apple-touch-icon.png`, PWA icons).
- **CI/CD & Packaging Pipeline**:
  - Single source of truth versioning: `scripts/setup.ps1` dynamically propagates `Cargo.toml` version to `Package.appxmanifest`.
  - Automated GitHub Actions workflows for continuous integration (`ci.yml`) and Microsoft Store submission (`publish.yml`).
  - Unit test `manifest_version_matches_cargo_pkg_version` enforcing version alignment.
- **Open Source Documentation Suite**:
  - Full project documentation including `README.md`, `PRIVACY.md`, `CONTRIBUTING.md`, `SECURITY.md`, and `CODE_OF_CONDUCT.md`.
  - Transparent disclosure explaining Windows "Precise Location" grouping for Wi-Fi SSID display.
