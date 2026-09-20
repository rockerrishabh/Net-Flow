---
layout: default
title: Privacy Policy
permalink: /privacy/
---

# Privacy Policy for Net Flow

**Last Updated**: September 17, 2026  
**Author / Maintainer**: Rishabh Kumar ([admin@rockerrishabh.me](mailto:admin@rockerrishabh.me))

Net Flow is an open-source, non-commercial, individual-maintained network telemetry monitor and native Windows 11 widget. This Privacy Policy outlines how Net Flow operates and explains our strict commitment to user privacy, data minimization, and transparency.

---

## 1. Summary: 100% Offline & Zero Data Collection

Net Flow adheres to a strict **zero-telemetry, zero-collection** policy:

- **No Outbound Network Requests**: Net Flow does not send telemetry, analytics, crash reports, or diagnostics over the internet. It makes **zero outbound network connections**.
- **No Personal Data Collection**: Net Flow does not collect, log, store, or transmit personal identifiers, browsing history, DNS queries, packet contents, IP addresses, or location data.
- **Local-Only Processing**: All bandwidth computations, telemetry waveform rendering, and process attribution occur exclusively in-memory on your local machine.

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
- The SSID is read solely to render the human-readable text label in the widget header and is never written to disk or sent anywhere.

### Running Without Location Permission
If you prefer not to grant location access:
1. Open Windows **Settings** → **Privacy & security** → **Location**.
2. Turn off location access globally or for desktop applications.
3. Net Flow will continue to operate with **100% functionality**. It gracefully falls back to displaying the generic adapter name (`Wi-Fi`) in the card header.

---

## 3. Local Data & Session Persistence

Net Flow persists minimal state locally on your computer to support its widget capabilities:

- **Widget Configuration**: User preferences configured in the card settings (selected speed unit, chart timeframe, and interface filters) are saved locally using standard Windows App SDK widget state storage.
- **Cumulative Session Counters**: Total bytes sent and received during the current session are saved locally in `%LocalAppData%` so your cumulative totals persist across system reboots or widget reloads.
- **Diagnostic Logging**: A lightweight, thread-safe diagnostic log (`%TEMP%\netflow_widget.log`) records widget lifecycle events (initialization, activation, shutdown). It is capped at 1 MB, automatically rotates to `.old`, is never transmitted off your machine, and can be deleted at any time.
- **Inline Reset**: You can clear cumulative session totals at any time by clicking the inline **Reset** button directly on the widget card.
- **Data Deletion**: Uninstalling the Net Flow package completely removes all associated local settings and session data from your machine.

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
