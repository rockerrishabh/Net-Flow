# Contributing to Net Flow

Thank you for your interest in contributing to Net Flow! Net Flow is an open-source project created and maintained by **Rishabh Kumar** ([@rockerrishabh](https://github.com/rockerrishabh)). 

Contributions of all forms—bug reports, documentation improvements, feature suggestions, and pull requests—are warmly welcomed.

---

## 📜 Licensing of Contributions

Net Flow is dual-licensed under:
- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

By submitting a pull request, patch, or code contribution to this repository, you explicitly agree that your contribution is licensed under this dual license without any additional terms or restrictions.

---

## 🤝 Code of Conduct

All contributors and participants are expected to adhere to our [Code of Conduct](CODE_OF_CONDUCT.md) to maintain a welcoming, respectful, and inclusive environment.

---

## 🛠️ Development Environment & Prerequisites

To build and test Net Flow locally, ensure you have:

1. **Windows 11** (Build 22000 or newer).
2. **Rust Toolchain (Stable)**: Edition 2024. Install via [rustup.rs](https://rustup.rs/):
   ```powershell
   rustup default stable
   rustup update
   ```
3. **Windows 10/11 SDK**: Required for `makeappx.exe` and `signtool.exe` (included with Visual Studio Build Tools or the standalone Windows SDK).
4. **PowerShell 7+ or Windows PowerShell 5.1**.

---

## 🚀 Getting Started

### 1. Fork & Clone
```powershell
git clone https://github.com/rockerrishabh/net-flow.git
cd net-flow
```

### 2. Run Tests
Ensure all workspace tests pass before making changes:
```powershell
cargo test --workspace
```

### 3. Run Clippy (Zero Warnings Policy)
The CI pipeline treats warnings as errors (`-D warnings`):
```powershell
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### 4. Build & Sideload to Windows 11 Widgets Board
To test changes live on your Windows 11 Widgets board:
```powershell
powershell -ExecutionPolicy Bypass -File scripts/setup_msix.ps1
```
This script will:
1. Compile the release binaries with embedded icons.
2. Generate a local self-signed developer certificate (if needed).
3. Pack and sign `NetFlow.msix`.
4. Sideload the package and refresh the Windows Widgets Board.

---

## 📐 Architecture & Design Principles

When proposing changes, keep the following core design principles in mind:

### Workspace Structure
- **`crates/core`**: OS-independent core library containing telemetry calculation (`backend.rs`), sparkline rendering (`chart.rs`), Adaptive Card JSON builders (`card.rs`), and vector icons (`icons.rs`). This crate contains no COM or widget runtime code.
- **`widget`**: The Windows App SDK widget provider implementing `IWidgetProvider2` via COM.
- **`scripts`**: Developer packaging and local MSIX sideloading automation (`setup_msix.ps1`).

### Core Guidelines
1. **Zero Elevation**: Net Flow runs strictly as a non-elevated standard user application. Never introduce dependencies requiring administrative or UAC privileges.
2. **No Raw Emojis in Adaptive Cards**: Windows Widgets Board renders emoji fonts inconsistently across scaling factors. All icons and badges in cards must use pre-rasterized vector PNG data URIs defined in [`crates/core/src/icons.rs`](crates/core/src/icons.rs).
3. **Single Source of Truth for Versions**: The project version is defined in [`Cargo.toml`](Cargo.toml). [`scripts/setup_msix.ps1`](scripts/setup_msix.ps1) and CI workflows automatically propagate this version to [`Package.appxmanifest`](widget/Package.appxmanifest).
4. **100% Offline & Private**: Do not add outbound network telemetry, analytics, or external tracking libraries. All telemetry processing must remain entirely local.

---

## 📝 Pull Request Checklist

Before submitting your pull request:
- [ ] Added unit tests covering new logic or bug fixes.
- [ ] Verified that `cargo test --workspace` passes cleanly.
- [ ] Verified that `cargo clippy --workspace --all-targets --all-features -- -D warnings` returns 0 warnings.
- [ ] Verified live widget functionality via `scripts/setup_msix.ps1`.
- [ ] Updated documentation or `README.md` if user-facing behavior changed.

---

## 📬 Need Help?

Have questions or want to discuss an idea before implementing it? Feel free to open a [GitHub Issue](https://github.com/rockerrishabh/net-flow/issues) or reach out directly to Rishabh Kumar at [admin@rockerrishabh.me](mailto:admin@rockerrishabh.me).
