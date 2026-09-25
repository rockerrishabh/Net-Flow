Net Flow is a lightning-fast, real-time network bandwidth monitor and telemetry widget designed natively for the Windows 11 Widgets Board and system tray. Built from the ground up in pure Rust, Net Flow delivers fluid, high-fidelity traffic visualization with near-zero CPU and memory overhead.

Whether you are gaming, streaming, downloading large files, or troubleshooting connection drops, Net Flow gives you instant, beautiful visibility into your network activity right from your taskbar and Windows Widgets Board.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✨ KEY FEATURES
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

• NATIVE WINDOWS 11 WIDGET INTEGRATION
  Seamlessly integrates into the Windows 11 Widgets Board (Win + W). Choose between Small, Medium, or Large card sizes to fit your workflow.

• REAL-TIME NETWORK ROUND-TRIP LATENCY
  Live ICMP ping measurement using asynchronous Win32 network APIs. Monitor round-trip time directly to your default gateway router or public internet (1.1.1.1) with unambiguous latency indicators (ms, timeout, offline).

• MIRRORED DUAL-STREAM WAVEFORMS
  Visualizes download and upload traffic simultaneously with peak-preserving Catmull-Rom spline smoothing, dynamic scale headroom, and glowing pulse nodes.

• PERSISTENT BACKGROUND SYSTEM TRAY HOST
  A lightweight, decoupled notification-area host starts automatically with Windows login, keeping continuous track of your bandwidth and latency even when widgets are unpinned. Hover over the tray icon for live throughput and latency tooltips.

• PER-APPLICATION BANDWIDTH ATTRIBUTION
  Real-time inspection of active applications consuming network bandwidth, showing individual process transfer rates and socket connection counts.

• INDEPENDENT BANDWIDTH ALERTS
  Configurable, independent download and upload thresholds. Receive native Windows toast notifications when sustained bandwidth exceeds your set limits.

• SMART MULTI-ADAPTER SUPPORT
  Automatically detects your primary active network interface (Wi-Fi with friendly SSID, Ethernet, Cellular, or VPN) and allows switching between adapters right from in-card settings.

• ACCURATE SESSION ACCOUNTING
  Authoritative cumulative upload/download data transferred and session duration with atomic persistence across restarts. Reset session anytime with one click.

• 3 GRAPH STYLES & THEMES
  Customize your sparkline with Area (waveform fill), Line (minimalist strokes), or Bar (discrete columns). Automatically tracks Windows Dark/Light mode.

• PURE RUST PERFORMANCE & EFFICIENCY
  Engineered for zero distraction and maximum efficiency: under 0.1% CPU usage, less than 15 MB RAM, and a compact standalone binary.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
🔒 PRIVACY FIRST
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

• 100% Offline & Private: No telemetry, no analytics, no external tracking, no accounts required.
• All statistics are queried directly from Windows local APIs (IP Helper & ICMP echo) and never leave your PC.
• Location permission note: Windows requires the location capability solely to query the Wi-Fi network SSID (network name) to display your connection status. Your physical location is never accessed, tracked, or stored.
