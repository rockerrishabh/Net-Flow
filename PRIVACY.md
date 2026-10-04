---
layout: default
title: Privacy Policy
permalink: /privacy/
---

# Privacy Policy for Net Flow

**Last Updated**: October 4, 2026

**Author / Maintainer**: Rishabh Kumar ([admin@rockerrishabh.me](mailto:admin@rockerrishabh.me))

Net Flow is an open-source, non-commercial, individual-maintained network telemetry monitor and native Windows 11 widget. This Privacy Policy outlines how Net Flow operates and explains our strict commitment to user privacy, data minimization, and transparency.

---

## 1. Summary: Local Processing & No Telemetry Uploads

Net Flow does not upload telemetry or analytics. It reads local network and process metadata to provide monitoring and diagnostics:

- **No Telemetry Uploads**: Net Flow does not send analytics, crash reports, or diagnostic reports to a server. Its latency monitor sends ICMP echo probes with the fixed payload `NetFlow` to the selected gateway or Cloudflare resolver (`1.1.1.1` / `2606:4700:4700::1111`). An Internet endpoint can observe your public IP address and probe timing; the probes contain no user or application traffic.
- **Local Network Details**: To display and export diagnostics, Net Flow reads local adapter details such as IP addresses, DNS servers, gateways, Wi-Fi identifiers, and active process names. These are processed and stored locally; diagnostic exports can contain them. Net Flow does not inspect browsing history or packet contents and does not send these details to a server.
- **Local-Only Processing**: Bandwidth computations, telemetry waveform rendering, and socket-owner estimates occur on your local machine. The app estimates per-app rates by dividing adapter throughput among processes according to their open socket counts; it does not read packet contents.

---

## 2. Why Windows Mentions "Precise Location"

When running Net Flow on Windows 11, Windows may show Net Flow under **Settings → Privacy & security → Location** or display a location icon in the taskbar notification area.

### Technical Explanation
- **Friendly Wi-Fi Network Name**: In the widget card header, Net Flow displays the name of your active Wi-Fi network (for example, `Home-5G` or `Office-Guest`) instead of an unhelpful generic device string.
- **The Win32 Wi-Fi API**: To read the active SSID, Net Flow queries the Windows Native Wi-Fi API (`wlanapi.dll` via `WlanQueryInterface`).
- **Microsoft's Privacy Grouping**: Because Wi-Fi network identifiers (SSIDs and BSSIDs) can theoretically be cross-referenced against global Wi-Fi positioning databases to estimate geographic position, Windows 10 and Windows 11 group all native Wi-Fi query APIs under the **"Precise Location"** permission umbrella.

### Our Guarantee
- Net Flow **does not track or estimate your geographic location**.
- Net Flow contains **no GPS code, no geolocation libraries, and no reverse-geocoding calls**.
- The SSID is used for the human-readable widget label and physical-link diagnostics. Wi-Fi physical-link data may be saved with local session state. It is not sent to a server; an export may include Wi-Fi identifiers.

### Running Without Location Permission
If you prefer not to grant location access:
1. Open Windows **Settings** → **Privacy & security** → **Location**.
2. Turn off location access globally or for desktop applications.
3. Net Flow will continue to operate with **100% functionality**. It gracefully falls back to displaying the generic adapter name (`Wi-Fi`) in the card header.

---

## 3. Local Data & Session Persistence

Net Flow persists minimal state locally on your computer to support its widget capabilities:

- **Widget Configuration**: User preferences configured in the card settings (selected speed unit, chart timeframe, selected adapter, alert thresholds and cooldowns, and data budget settings) are saved locally in the Net Flow configuration file under `%LocalAppData%`.
- **Cumulative Session Counters**: Total bytes sent and received during the current session are saved locally in `%LocalAppData%` so your cumulative totals persist across system reboots or widget reloads.
- **Diagnostic Logging**: A lightweight, thread-safe diagnostic log (`%TEMP%\netflow_widget.log`) records widget lifecycle events (initialization, activation, shutdown). It is capped at 1 MB, automatically rotates to `.old`, is never transmitted off your machine, and can be deleted at any time.
- **Inline Reset**: You can clear cumulative session totals at any time by clicking the inline **Reset** button directly on the widget card.
- **Data Deletion**: Configuration, session history, and diagnostics logs are stored in local app-data or temporary folders and may remain after uninstall. To remove them, delete `%LocalAppData%\NetFlow` and `%TEMP%\netflow_widget.log` (and its rotated `.old` file).

---

## 4. Third-Party Services & Dependencies

Net Flow is an independent open-source project and does not integrate with any third-party analytics, advertising networks, tracking SDKs, or cloud services. 

---

## 5. Security & Elevation

- Net Flow runs strictly as a **non-elevated standard user application**.
- It does **not** require administrator privileges (UAC elevation).
- Network transfer rates are polled via read-only Windows IP Helper APIs (`GetIfTable2`), ensuring your system configuration is never modified.

---

## 6. Open Source Transparency

Because Net Flow is open-source under the **MIT OR Apache-2.0** dual license, the entire codebase is publicly auditable. You can inspect the complete source code, review API calls, and build the project from scratch at:

👉 [https://github.com/rockerrishabh/Net-Flow](https://github.com/rockerrishabh/Net-Flow)

---

## 7. Contact & Questions

If you have any questions, concerns, or feedback regarding this Privacy Policy or Net Flow's privacy practices, please contact:

- **Maintainer**: Rishabh Kumar
- **Email**: [admin@rockerrishabh.me](mailto:admin@rockerrishabh.me)
- **GitHub Issues**: [https://github.com/rockerrishabh/Net-Flow/issues](https://github.com/rockerrishabh/Net-Flow/issues)
