# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

---

## [0.1.0] - 2026-09-17

### Added

- **Native Windows 11 Widgets Board Integration**:
  - Full support for Small, Medium, and Large card sizes using Adaptive Cards v1.6.
  - Interactive in-card settings for speed units (Auto-scaled, Bytes/s, KB/s, MB/s, Gbps) and timeframes (15s, 30s, 60s, 120s).
- **Dual-Stream Telemetry Waveforms**:
  - In-memory supersampled rasteriser rendering mirrored sparklines on a shared scale (cyan download above baseline, amber upload below).
  - Anti-aliased area gradient fills and glowing live pulse indicator nodes.
  - 50 KB/s scale floor (`MIN_CHART_SCALE_BPS`) keeping sub-kilobyte background noise proportional.
  - 18% vertical headroom preventing peak traffic spikes from touching card boundaries.
- **Active App Bandwidth Attribution**:
  - Per-process network throughput monitoring combining socket tracking and I/O heuristics.
  - Compact right-column rate notation with shared bandwidth units (`↓ 11.1 ↑ 9.1 KB/s (37)`).
  - Generous 22-character application name budget on Medium and Large cards to prevent truncation.
- **Smart Interface Detection**:
  - Auto-detection of primary active network interface (Wi-Fi, Ethernet, Cellular, VPN).
  - Safe Wi-Fi SSID resolution via Windows Native Wi-Fi API (`wlanapi.dll`) with graceful fallback.
- **Session Telemetry & Persistence**:
  - Cumulative upload/download session counters and elapsed duration tracking.
  - Inline **Reset** action directly on the card to reset session statistics.
  - Local session state persistence across reboots in `%LocalAppData%`.
- **High-Performance Release Profile**:
  - Configured `[profile.release]` with `opt-level = 3`, `lto = true`, `codegen-units = 1`, `strip = true`, and `panic = "abort"`.
  - Achieved a compact **1.24 MB** standalone release binary.
- **Packaging & Publishing Pipeline**:
  - Store-ready MSIX packaging pipeline for automated Microsoft Partner Center ingestion.
  - Automated GitHub Actions workflows for continuous integration (`ci.yml`), GitHub Releases (`release.yml`), and store publishing (`store-publish.yml`).
  - Single source of truth versioning automatically synchronized from `Cargo.toml`.
- **Comprehensive Test Suite**:
  - 75 automated unit tests covering bandwidth allocation math, history ring buffers, card templates, and lock poison recovery.
  - Strict zero-warning Clippy policy enforced across all workspace targets.

### Changed

- **Decoupled Process Sampling Cadence**:
  - Separated heavy Win32 process queries (`CreateToolhelp32Snapshot`, `GetProcessMemoryInfo`) to a 1.0-second interval.
  - Maintained continuous 500ms polling for physical network interface throughput (`GetIfTable2`), eliminating system scheduler jitter and bandwidth scaling distortion.
- **Lock Poison Resilience**:
  - Implemented extension traits (`LockExt`, `RwLockExt`) providing `.lock_safe()`, `.read_safe()`, and `.write_safe()` across all mutexes and read-write locks.
  - Guarantees automatic poison recovery across worker threads under `panic = "abort"`.
- **COM Server Lifetime & Idle Shutdown**:
  - Added idle monitoring loop with `has_had_widgets` tracking and a 30-second post-removal grace period.
  - Implemented graceful shutdown sequence: calls `CoRevokeClassObject`, terminates background worker thread, invokes `CoUninitialize()`, and cleanly exits to prevent zombie background processes.
- **Eviction-Bounded Icon Cache**:
  - Implemented a 256-entry capacity limit on `ICON_CACHE` in `crates/core/src/process.rs` to prevent memory growth during long-running sessions.
- **Lossy Wi-Fi SSID Decoding**:
  - Switched from strict UTF-8 decoding to `String::from_utf8_lossy` to safely handle non-standard SSID encodings without panicking.
- **Thread-Safe Rotating Logging**:
  - Synchronized file I/O under `LOG_MUTEX`.
  - Capped widget log size at 1 MB, automatically rotating `netflow_widget.log` to `.old`.
  - Throttled verbose 1Hz card generation ticks behind `NETFLOW_VERBOSE_LOG=1`.
