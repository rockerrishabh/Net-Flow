# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.0] - 2026-10-03

### Added

- **Diagnostic Telemetry & Export Engine**:
  - Pure-Rust canonical diagnostic data model (`schema_version = 1`) in `crates/core/src/export.rs` capturing deep network telemetry, multi-adapter configurations, session metrics, ICMP quality, and 90-day daily usage history.
  - Strongly typed enums for `Architecture` (`x64`, `arm64`, `unknown`), `InterfaceType` (`ethernet`, `wifi`, `cellular`, `vpn`, `loopback`, `other`), `SemanticHealth` (`healthy`, `degraded`, `timeout`, `unavailable`), and `WifiGeneration` (`wifi_7`, `wifi_6e`, `wifi_6`, `wifi_5`, `wifi_4`, `unknown`).
  - Structured machine-readable privacy metadata (`PrivacyMetadata`) declaring exposure of local network identifiers, Wi-Fi parameters, and active process names.
  - Deterministic 5-point Active Adapter Selection Policy prioritizing `IfOperStatusUp`, Default Gateway presence, usable non-APIPA IP, physical interfaces over virtual/software adapters, and lowest routing metric.
  - Dual document exporters:
    - High-fidelity canonical JSON exporter (`export_to_json`) supporting pretty-printing and compact streaming.
    - Specialized Net Flow CSV dialect (`export_to_csv`) with `# CSV Dialect: netflow-diagnostics-v1` header block and strict RFC 4180 CRLF rows with field quoting and escaping.
  - Exact mean absolute RTT difference jitter calculation: $\text{Jitter} = \frac{1}{N-1} \sum_{i=2}^N |\text{RTT}_i - \text{RTT}_{i-1}|$ explicitly labeled and calculated without conflating with RFC 3550 RTP interarrival jitter.
  - 90-day historical continuity padding (`zero_fill_history`) guaranteeing continuous chronological day-by-day records ending on the current local calendar date.
- **Headless CLI Diagnostic Export**:
  - Command-line diagnostic extraction interface: `net-flow.exe --export <csv|json> [path]`.
  - Stdout pipeline mode: `net-flow.exe --export json -` writes exclusively to stdout, routing all logs and diagnostic messages strictly to stderr for clean pipe composition (`| jq .`).
  - Headless atomic file writing with zero GUI dialogs or interactive prompts.
- **Companion Flyout 3-Button Action Bar**:
  - Transitioned Win32 companion flyout action bar to a balanced 3-button layout: `[Export]`, `[Reset]`, `[Close]`.
  - Native Win32 GDI rendering with DPI-scaled hit-testing and hover tracking for all 3 buttons across 100%, 125%, 150%, and 200% scaling.
  - Clicking `[Export]` directly opens the native Save As dialog defaulting to the Downloads folder.
- **System Tray Diagnostics Menu**:
  - Context menu items: `Export Diagnostics (CSV)...` and `Export Diagnostics (JSON)...` with native file dialog invocation.
- **Secure File Writing & Toast Activation**:
  - Atomic disk writing via `.netflow_export.<id>.tmp` buffer, flushed and synced, then atomically replaced via Win32 `MoveFileExW` (`MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH`) with graceful rename fallback.
  - In-memory bounded token registry (max 16 entries, 1-hour TTL) mapping opaque tokens (`action=open-file&token=<uuid>`) to canonical paths.
  - Path validation verification ensuring that toast notification arguments cannot trigger arbitrary execution or directory traversal.

## [0.8.0] - 2026-10-03


### Added

- **Native Win32 Tray Flyout Companion (<5 MB RAM)**:
  - Ultra-lightweight native Win32 popup companion window (`WS_POPUP`, `WS_EX_TOPMOST | WS_EX_TOOLWINDOW`) toggled by left-clicking the notification area icon (`WM_LBUTTONUP`).
  - Pure GDI double-buffered rendering engine (`CreateCompatibleDC` + `CreateCompatibleBitmap` blitted to screen DC on `WM_PAINT`) with zero XAML, WinUI, or DirectComposition runtime overhead (<4 MB working set).
  - Strict Presentation Surface isolation: UI thread never queries `IpHlpApi`, `Ndis`, `ICMP`, WLAN, or process APIs; it renders strictly from an immutable `FlyoutSnapshot` prepared by `TrayWorker`.
  - Robust state machine lifecycle: explicit `Hidden -> Opening -> Visible -> Closing -> Hidden` transitions eliminating race conditions between tray clicks, window activation, and focus loss.
  - Blur & click-away dismissal: auto-dismissal on `WM_ACTIVATE` (`WA_INACTIVE`) and `WM_KILLFOCUS` with debounce protection against premature dismissal on opening.
  - Precision taskbar positioning hierarchy: primary query via `Shell_NotifyIconGetRect` for exact notification icon bounding rectangle, with `SHAppBarMessage(ABM_GETTASKBARPOS)` fallback and work area clamping via `GetMonitorInfoW(rcWork)` across multi-monitor setups.
  - Modern Windows 11 DWM chrome: clean abstraction `apply_window_chrome` setting `DWMWA_USE_IMMERSIVE_DARK_MODE` (20) and `DWMWA_WINDOW_CORNER_PREFERENCE` (33) with `DWMWCP_ROUND` (2).
  - Per-monitor DPI awareness: logical DIPs (`FLYOUT_WIDTH_DIP = 328`, `FLYOUT_HEIGHT_DIP = 456`) scaled dynamically via `GetDpiForWindow`, handling `WM_DPICHANGED` by resizing backbuffers and typography.
  - Rich companion telemetry dashboard:
    - Branded header with active medium and link summary badge (e.g. Ethernet / Wi-Fi).
    - Large download and upload metric cards with distinct emerald and blue accents.
    - Latency, jitter, packet loss percentage, and health status pill with colored indicators.
    - Live 30-sample GDI sparkline line chart visualizing download and upload traffic rails.
    - Session bandwidth totals with compact duration formatting.
    - Data budget quota status and proportional progress bar with threshold color coding.
    - Top 3 active network-consuming applications with live download/upload throughput attribution.
    - Interactive quick-action buttons ("Reset Session" and "Close") with mouse hover tracking and click hit-testing.
- **Windows on ARM64 Platform Hardening**:
  - Full Tier-1 native target support for `aarch64-pc-windows-msvc`.
  - Compile-time architecture constant `ARCH` (`x64` / `arm64`) with unit testing.
  - Parameterized `AppxManifest.xml` generation supporting dynamic processor architecture injection.
  - Multi-tier CI: pull request validation across both `x86_64` and `aarch64`, release builds, and manifest validation.
  - Dual release packaging: automated generation of `net-flow-windows-x64.zip` and `net-flow-windows-arm64.zip` with SHA256 checksums and dual MSIX Store submission pipeline.
- **Comprehensive Test Suite**:
  - Flyout geometry calculation unit tests covering bottom, top, left, and right taskbars.
  - Secondary monitor screen-edge work-area clamping tests.
  - DPI DIP-to-physical scaling tests.
  - Short duration and rate formatting tests.
  - Button hit-testing tests.
  - State machine lifecycle transition tests.
  - 100-cycle snapshot memory stability test verifying zero monotonic heap accumulation.

---

## [0.7.1] - 2026-09-29

### Fixed

- **Widget Customization Flyout Viewport Safety**:
  - Restructured the settings card into a compact, table-aligned 3-column layout (`Bandwidth alerts` and `Data budget & quota`).
  - Reduced total vertical height from ~500px down to ~330px, providing ~150px of safety headroom inside the fixed-height Windows 11 Widgets Board dialog.
  - Eliminated vertical clipping that prevented the Monthly Cap input, Renewal Day input, Scope dropdown, and Save/Cancel buttons from being displayed.
  - Removed duplicate "Reset session" action from the settings dialog in favor of the one-click `[⟳]` button in the card session footer and the system tray context menu.

---

## [0.7.0] - 2026-09-29

### Added

- **Data Budgeting & Billing Cycle Quota Management**:
  - Configurable data cap allowance in gigabytes (`cap_gb`), custom monthly billing cycle renewal day (1–31), and tracking scope (`AllInterfaces`, `MeteredOnly`, `WifiOnly`, `EthernetOnly`).
  - Formalized monthly billing cycle boundaries (`cycle_start <= date < cycle_end`) with dynamic clamping to actual days in the month (`min(configured_day, days_in_month)`), full leap-year support, and year-wrap handling.
  - Zero-truncation `usage_pct: u16` supporting >100% over-budget states while providing clamped 0..=100 progress widths for UI cards.
- **Bounded Day-Keyed Historical Rolling Usage Store (`daily_usage.json`)**:
  - Mutable, sorted time-series keyed by unique local calendar date (`YYYY-MM-DD`) with automatic duplicate coalescing.
  - Automatic 90-day retention pruning discarding older entries on date rollover.
  - Batched in-memory accumulation on sampling ticks flushed atomically to disk every 30 seconds, on process shutdown, date rollover (midnight), and configuration update.
  - Atomic persistence using process-unique temporary files and atomic renames, preventing file corruption across sudden reboots or sleep/resume cycles.
- **Robust Interface Counter Discontinuity & NIC Reset Protection**:
  - Associating byte baselines with stable 64-bit `InterfaceLuid` keys in `HashMap<InterfaceLuid, InterfaceCounterState>`.
  - Explicit counter discontinuity policy (`counter_delta(previous, current) -> Option<u64>`) attributing 0 bytes on counter wraps, adapter resets, driver restarts, or sleep/resume instead of corrupting user budgets with massive false deltas.
- **Crossing-Based Multi-Tier Quota Alerts**:
  - Crossing-based milestone evaluation (`previous < threshold && current >= threshold && !notified`) for 80%, 90%, and 100% data budget thresholds.
  - Core-owned evaluation producing `Option<BudgetMilestone>` in `BudgetSnapshot`, decoupling alert logic from presentation hosts.
  - Multi-tier Windows toast notifications delivered via clean Windows notification XML (`show_budget_alert`), with cycle reset detection resetting milestone notification flags on each new billing cycle.
- **Adaptive Card Data Budget Progress & Customization Settings**:
  - Large widget card container with dual-column proportional progress bar (`budgetUsedWidth` and `budgetRemainingWidth`), semantic color styling (`Accent` <80%, `Warning` 80–99%, `Attention` ≥100%), consumed vs total volume text, and remaining billing cycle days.
  - Medium widget card compact badge (`58% of 500 GB · 12d left`).
  - Native widget settings card controls for toggling data budget, setting allowance cap (GB), renewal day (1–31), and interface scope.
- **System Tray Tooltip Quota Status**:
  - Live tooltip now displays real-time budget quota status (e.g. `Budget: 289.4 GB / 500 GB (58%) · 12d left`).

---

## [0.6.0] - 2026-09-29

### Added

- **Dual-Stack IPv6 & IPv4 Probing**:
  - Native Win32 ICMPv6 telemetry via `Icmp6CreateFile`, `Icmp6SendEcho2`, and `Icmp6ParseReplies` using a platform-neutral internal abstraction `probe_latency(target, timeout_ms)`.
  - Automatic IPv6 preference with seamless IPv4 fallback, querying best route to Cloudflare DNS (`2606:4700:4700::1111`) via `GetBestRoute2` to resolve source IP and interface index.
  - Clear semantic distinction between successful ping responses, timeouts, and route unavailabilities.
- **Rolling Circular Packet Loss % & Semantic Latency Health**:
  - Authoritative 20-sample rolling circular buffer (~40-second window) maintained exclusively in `backend.rs` as the single source of truth for both widget cards and system tray.
  - Proper packet loss calculation that excludes route unavailabilities from dropped packet counts.
  - Explicit semantic health classifications: `Healthy`, `Degraded`, `Timeout`, and `Unavailable`.
- **Physical Link Telemetry & Wi-Fi 7 / MLO Data Model**:
  - Deep Wi-Fi physical layer telemetry using Windows Native Wi-Fi API (`wlanapi.dll`) with `WLAN_REALTIME_CONNECTION_QUALITY` (opcode 19), requiring zero location permissions.
  - Pure deterministic 802.11 generation mapping from `DOT11_PHY_TYPE` (Wi-Fi 7/EHT, Wi-Fi 6/HE, Wi-Fi 5/VHT, Wi-Fi 4/HT, Legacy) and documented RSSI dBm calculation (`(quality / 2) - 100`).
  - Active Multi-Link Operation (MLO) connection tracking (`is_mlo`, `link_count`).
  - Dynamic Ethernet link speed detection via IP Helper `MIB_IF_ROW2` (`tx_speed_bps`, `rx_speed_bps`) formatted cleanly (e.g. `Ethernet · 1 Gbps`).
- **Adaptive Card Physical Link Slot & Hierarchical Header Layout**:
  - Conditional physical link container on Large widget cards utilizing Adaptive Cards 1.6 `$when: "${hasPhysicalLink == true}"`.
  - Clean semantic latency hierarchy across card sizes:
    - Small: `18 ms` (or `Timeout`/`Unavailable`)
    - Medium: `18 ms · 0% loss`
    - Large: `18 ms · ±2 ms · 0% loss` with Physical Link line displayed cleanly below.
- **Enriched System Tray Tooltip**:
  - Live tooltip now displays authoritative packet loss % and active physical transmission link summary while strictly respecting the 127-character `szTip` buffer limit.

---

## [0.5.0] - 2026-09-28

### Added

- **Windows App SDK 2.x Modern Runtime**:
  - Migrated package framework dependencies from legacy 1.7 baseline to modern `Microsoft.WindowsAppRuntime.2` (`MinVersion="2.0.0.0"`).
  - Upgraded CI & release pipelines to Windows App SDK 2.5.1 with automated multi-nupkg WinMD dependency extraction for `Microsoft.WindowsAppSDK.Widgets`.
- **Ping Jitter & Real-Time Latency Waveforms**:
  - Real-time round-trip latency variance (jitter in ms) tracked continuously by the background ICMP telemetry engine.
  - Dedicated subtle purple latency sparkline overlaid on Medium and Large widget cards, keeping Small cards clean and minimal.
  - Diagnostic latency metrics formatted with jitter spread (e.g. `18 ms (±2 ms) · Internet`).
- **Expanded System Tray Host Controls**:
  - Display and toggle Windows login startup status directly from the tray context menu (`Run at startup: Enabled / Disabled`) via non-blocking WinRT `StartupTask` API calls.
  - Quick-cycle latency target mode (`Auto` → `Internet` → `Gateway`) directly from the notification tray without opening settings.

### Changed

- **Native Windows 11 Widget Customization UX**:
  - Removed duplicate in-card gear icon from live widget headers in favor of Windows 11 Widget Board's native "Customize widget" flyout.
  - Standardized settings card actions with positive/cancel `ActionSet` buttons.
  - Focused graph presentation styles on high-contrast `Area (Waveform)` and `Bar (Columns)`.

---

## [0.4.2] - 2026-09-25

### Changed

- **Streamlined Settings Flyout**:
  - Removed the redundant "All active adapters" dropdown from in-card settings across all widget sizes. Net Flow now seamlessly and automatically monitors your active network interfaces without requiring manual adapter configuration.

---

## [0.4.1] - 2026-09-25

### Changed

- **Clean In-Card Settings Flyout**:
  - Removed redundant "Active apps" toggle section from the settings card to optimize vertical layout and prevent content clipping in Windows Widget flyouts.
  - Active apps drawer expansion remains directly accessible and interactive via the dedicated chevron button on the main widget card.

### Fixed

- **Adaptive Card Input Schema Compliance**:
  - Corrected `Input.Number` default values to emit pure numeric primitives instead of serialized strings, adhering strictly to Adaptive Cards v1.6 specification.

---

## [0.4.0] - 2026-09-25

### Added

- **Route-Driven Asynchronous Win32 ICMP IPv4 Latency Telemetry**:
  - Implemented asynchronous IPv4 ICMP ping probing utilizing Win32 `IcmpCreateFile`, `CreateEventW`, `IcmpSendEcho2`, `WaitForSingleObject` (1-second timeout), and `IcmpParseReplies` on a 2-second background cadence.
  - Implemented route-driven IPv4 default gateway discovery via `GetBestRoute2` towards public internet (`1.1.1.1`) querying the next hop Windows actually uses, with graceful `GetIpForwardTable2` fallback.
  - Added configurable endpoint target selector (`Auto`, `Internet` [1.1.1.1], `Gateway` [default router]) in widget settings with stable semantic state handling: route availability determines probe target, while timeouts represent measurement states.
  - Integrated unambiguous, semantic latency displays across all card headers (`18 ms`, `Timeout`, `-- ms`) alongside active connection counts, with detailed diagnostic labels in tooltips and session summaries.
- **Decoupled Persistent Background System Tray Host**:
  - Decoupled the notification-area host into a persistent background process (`net-flow.exe --tray`), owned directly by Windows login via `<uap5:StartupTask>` in `Package.appxmanifest`.
  - Single-instance enforcement via named mutex `Global\NetFlow_Tray_Mutex` with fallback to `Local\NetFlow_Tray_Mutex`.
  - Self-healing recovery: Widget COM host inspects tray mutex on startup and spawns detached `net-flow.exe --tray` if the tray is missing.
  - Live tooltip formatted with aggregate transfer rates and round-trip latency (`Net Flow\n↓ {dl}  ↑ {ul}\nLatency: {ms} ({target})`).
  - Context menu with "Open Widgets Board", "Reset session totals", and clean "Exit Net Flow".
- **Tray-Authoritative Session State & Atomic Consistency**:
  - Made the persistent Tray Host the single authoritative source of cumulative bandwidth accounting, ICMP latency telemetry, and alert state machines.
  - Replaced independent accumulators with atomic file persistence (`session_state.json`) utilizing process-unique temporary files, flush/sync, and retry backoff.
  - Added monotonic `generation` counter and Win32 named event `Global\NetFlow_ResetSession_Event` for instant, race-free session resets synchronized across widget and tray instances.
- **Asymmetric, Independent Download and Upload Bandwidth Alerts**:
  - Replaced shared sustain timers with completely independent, concurrent download and upload state machines.
  - Added separate configurable thresholds (MiB/s), sustain durations (seconds), and cooldown windows for download and upload traffic, with backward-compatible configuration deserialization.
  - In-card settings across Small, Medium, and Large widgets updated with independent download and upload toggle and threshold controls.
- **Visual Polish & Sparkline Smoothing**:
  - Refined sparkline Catmull-Rom smoothing to anchor the leading edge to zero while strictly preserving local peak magnitudes.
  - Added bold bar tracks for the discrete Bar graph style.

---

## [0.3.0] - 2026-09-24

### Added

- **Bandwidth Usage Alerts (Sustained-Threshold Toast Notifications)**:
  - Added a platform-independent alert engine (`BandwidthAlertEngine`) that fires only after a direction stays above its threshold for a configurable sustain window, with a per-direction cooldown that rate-limits duplicate notifications.
  - Delivered native Windows toast notifications for packaged apps via `ToastNotificationManager`, using the MSIX package identity (no AppUserModelID shortcut required).
  - Added in-card alert settings — a "Notify on sustained high download" toggle plus threshold (MiB/s) and sustain (seconds) inputs — across Small, Medium, and Large widgets.
  - Persisted alert preferences globally to `%LocalAppData%\NetFlow\alerts.json`, mirrored into the live worker engine on save so they survive widget recreation and process restarts.
- **Multiple Adapter Support**:
  - Extended the telemetry backend to track every active interface simultaneously (`per_interface` samples sorted by throughput), while aggregate totals remain unchanged.
  - Added an adapter selector to widget settings ("All active adapters" plus up to 16 named interfaces) driving headline download/upload metrics and peaks for the chosen LUID.
  - Added a flicker-free "Active adapters" breakdown on the Large card using four fixed data-bound slots (`adapter0`..`adapter3`) with `$when` visibility.
- **System Tray Icon (Notification Area)**:
  - Added an in-process tray icon via `Shell_NotifyIconW` on a dedicated message-loop thread, registered with a stable GUID as Microsoft recommends.
  - Live tooltip refreshes ~1 Hz with current aggregate download/upload rates.
  - Right-click context menu with "Reset session totals" (mirrors the in-card Reset action) and "Exit Net Flow".
  - Scoped to the widget host lifetime: the icon is added on activation and removed with `NIM_DELETE` during idle shutdown, preserving the existing clean-exit behaviour. Isolated behind a small `TrayIcon` (`spawn` / `destroy`) abstraction for future relocation to a persistent host.

---

## [0.2.1] - 2026-09-21

### Changed

- **Automatic Widgets Board Theme Synchronization**:
  - Removed manual in-card theme selector from widget settings; the card automatically tracks the system Widgets Board surface.
  - Enhanced theme detection in `query_windows_light_theme` to prioritize `SystemUsesLightTheme` (governing the Windows 11 Shell and Widgets Board), falling back to `AppsUseLightTheme`.
  - Streamlined the Settings card layout across Small, Medium, and Large widgets with cleaner spacing.

---

## [0.2.0] - 2026-09-21

### Added

- **Auto-Start on Boot (Windows StartupTask)**:
  - Added packaged Win32 `<uap5:StartupTask>` declaration in `Package.appxmanifest` (`NetFlowStartup`), enabling Net Flow to initialize on Windows user login.
  - Full user toggle control in Windows Settings (`Apps > Startup`) and Task Manager (`Startup apps`).
  - Graceful lifecycle integration: verifies pinned widgets on startup, maintaining active telemetry when pinned or cleanly terminating after the idle grace period if unpinned.
- **Dark/Light Theme-Aware Sparkline Waveforms**:
  - Implemented automatic Windows theme detection querying `AppsUseLightTheme` in `HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize` with graceful Dark default fallback.
  - Added dual-palette sparkline rendering:
    - **Dark Theme**: Electric cyan `#38D9F0` (DL) and warm amber `#FFB020` (UL) with subtle gray baseline.
    - **Light Theme**: Deep high-contrast cyan `#008CB4` (DL) and rich warm amber `#D75F00` (UL) with defined baseline, achieving WCAG AA contrast on light cards.
  - Added `Theme` dropdown (`Auto`, `Dark`, `Light`) in widget settings.
  - Extended immutable `OnceLock` idle chart cache across resolved themes to immediately reflect OS theme changes without stale cache collisions.
- **Graph Style Customization**:
  - Implemented modular sparkline renderers:
    - **Area**: Mirrored Catmull-Rom spline with smooth gradient area fill and leading pulse indicator dot.
    - **Line**: Clean, minimalist spline stroke with pulse dot without gradient area fill.
    - **Bar**: Discrete vertical bandwidth bars per sample with rounded heads from baseline.
  - Added in-card `Graph style` selector (`Area`, `Line`, `Bar`) across Small, Medium, and Large widgets.
- **Rendering Smoothness & Flicker Elimination**:
  - **Strict Template / Data Separation (Phase A)**: Visual templates (`SetTemplate`) are transmitted exclusively on structural lifecycle events (create, resize, settings enter/exit), while 500 ms telemetry updates strictly transmit dynamic data (`SetData`). In accordance with Windows Widgets API semantics, the host visual tree is retained without periodic reconstruction or layout tearing.
  - **Adaptive Card Templating & `$data` Repeater**: Dynamic values bind to `${downloadRate}`, `${uploadRate}`, `${peakRate}`, and `${chartUrl}`; the active applications list uses the Adaptive Card `$data` data-context/repeater bound to `${activeApps}`.
  - **Collision-Free Payload Diffing (Phase B)**: Evaluates a fast 64-bit hash combined with exact JSON string comparison per widget. Unchanged payloads during idle network traffic bypass `UpdateWidget` calls, eliminating idle host re-evaluations and cutting IPC traffic by up to 96%.
  - **Forced Action Publication**: App paging, session resets, and settings actions trigger immediate publication via `force_data_publish`.
- **Flicker-Free Active Apps Drawer**:
  - Replaced dynamic `$data` collection repeater with permanent fixed slots (`app0`..`app3` on Medium, `app0`..`app7` on Large) governed by `$when` visibility bindings, completely eliminating asynchronous image decode micro-blinking.
- **Adaptive Cadence (Traffic-Proportional Rate)**:
  - Dynamically throttles UI telemetry updates based on throughput: 500 ms during high traffic bursts (≥ 250 KiB/s), 1000 ms during moderate traffic, and 1500 ms during near-idle intervals (< 10 KiB/s), reducing background resource consumption by up to 60%.
- **Tactile Button Containers & Fluent Hover Effects**:
  - Explicit 28×28px column hit targets with `roundedCorners: true` for Settings gear, pagination chevrons, drawer toggle, and session reset controls.
  - Symmetrical 56×28px / 48×28px rounded pill hit targets for Cancel and Save in Settings card with balanced spacing.
- **Justify-Between Metrics Symmetry**:
  - Right-aligned Upload metrics (`↑ Upload`, rate, peak) to match the header row's justify-between spread, balancing Download on the left and Upload on the right.
- **Backward-Compatible Configuration Deserialization**:
  - Guaranteed seamless deserialization and migration of existing `CustomState` configurations from v0.1.0/v0.1.1.

---

## [0.1.1] - 2026-09-20

### Changed

- **Telemetry & UI Cadence Decoupling**:
  - Decoupled high-fidelity telemetry sampling (250 ms / 4 Hz) from widget card publication (500 ms / 2 Hz).
  - Expanded rolling history buffer from 60 to 240 samples for high-resolution waveform tracking without increasing UI load.
- **Incremental O(1) Chart Peak Tracking**:
  - Maintained running maximum download and upload peaks in `NetworkBackend`.
  - Replaced O(N) full-history scans on every UI frame with O(1) updates, only rescanning on FIFO eviction if the evicted sample matched the peak.
- **Sparkline Rasterizer Optimizations**:
  - Added zero-alpha destination fast path in `Canvas::blend` and unrolled RGB compositing channels.
  - Hoisted invariant scales and precomputed column metrics in `draw_track`, adding a dedicated branchless interior-row filling loop.
  - Accelerated downsampler by skipping transparent destination sub-pixels.
  - Reduced active chart rasterization latency by 45% (from 6.52 ms to 3.58 ms).
- **Precomputed Idle Chart Cache**:
  - Implemented immutable `OnceLock` cache for standard widget size and time window presets.
  - Enabled O(1) idle bypass when history peaks are zero, eliminating rasterization during idle periods (down from 6.68 ms to 120 us, a 55x speedup; total idle CPU overhead reduced to 0.024% of 1 core).
- **Official URLs**:
  - Updated official product homepage to `https://netflow.rockerrishabh.me`.
  - Configured dedicated privacy policy route at `https://netflow.rockerrishabh.me/privacy`.


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
