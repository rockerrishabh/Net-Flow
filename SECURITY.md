# Security Policy

## Supported Versions

Net Flow is maintained as an active open-source project. Security fixes and patches are provided for the latest release series:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1.0 | :x:                |

---

## Reporting a Vulnerability

If you discover a security vulnerability or potential exploit in Net Flow, please report it privately and responsibly so a fix can be prepared before public disclosure.

### Preferred Method: GitHub Private Vulnerability Reporting
Please submit an advisory through GitHub:
👉 [https://github.com/rockerrishabh/net-flow/security/advisories/new](https://github.com/rockerrishabh/net-flow/security/advisories/new)

### Direct Contact
If you cannot use GitHub Advisories, you can email the maintainer directly:
- **Contact**: Rishabh Kumar
- **Email**: [admin@rockerrishabh.me](mailto:admin@rockerrishabh.me)
- **Subject Line**: `[SECURITY] Net Flow Vulnerability Report`

### What to Include in Your Report
To help us investigate and remediate the issue promptly, please include:
1. Description of the vulnerability and its potential security impact.
2. Steps to reproduce the issue, proof-of-concept code, or minimal demonstration.
3. Affected system environment (Windows 11 build number, widget host version).
4. Any proposed remediations or patches, if available.

### Response Timeline
- **Initial Acknowledgment**: Within 48 hours of receipt.
- **Triage & Assessment**: Within 5 business days with a remediation plan or severity assessment.
- **Disclosure Policy**: We coordinate public disclosure after a patch has been verified, packaged, and released.

---

## Security Model & Threat Assessment

Net Flow is designed with a defense-in-depth security model:

1. **Non-Admin Execution**: Net Flow runs entirely in user-mode as a standard user process. It never requires or requests administrative elevation (UAC).
2. **Read-Only System APIs**: Network transfer statistics are collected via read-only Windows IP Helper APIs (`GetIfTable2`). It does not install system filter drivers or modify network routing.
3. **Memory Safety**: The core telemetry rasteriser, JSON card templating engine, and state tracking are implemented in pure Rust, providing compiler-enforced guarantees against memory corruption, buffer overflows, and use-after-free vulnerabilities.
4. **Zero Remote Attack Surface**: Net Flow opens no listening network sockets and makes no outbound network connections.
