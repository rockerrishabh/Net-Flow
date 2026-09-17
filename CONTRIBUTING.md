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
Ensure all 75 workspace unit tests pass before making changes:
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
1. Compile the release binaries with embedded icons and LTO optimizations.
2. Generate a local self-signed developer certificate (if needed).
3. Pack and sign `NetFlow.msix`.
4. Sideload the package and refresh the Windows Widgets Board.

---

## 📐 Architecture & Engineering Standards

When proposing changes, keep the following core design principles in mind:

### Workspace Structure
- **`crates/core`**: OS-independent core library containing telemetry calculation (`backend.rs`), sparkline rendering (`chart.rs`), Adaptive Card JSON builders (`card.rs`), vector glyphs (`icons.rs`), and process attribution (`process.rs`). This crate contains no COM or widget runtime code.
- **`widget`**: The Windows App SDK widget provider implementing `IWidgetProvider2` and COM class factories.
- **`scripts`**: Developer packaging and local MSIX sideloading automation (`setup_msix.ps1`).

### Core Engineering Guidelines

1. **Zero Elevation**: Net Flow runs strictly as a non-elevated standard user application. Never introduce dependencies requiring administrative or UAC privileges.
2. **COM FFI Boundary Safety & Lock Poison Resilience**:
   - Never use raw `.lock().unwrap()` or `.read().unwrap()`. Always use our poison-resilient extension traits: `.lock_safe()`, `.read_safe()`, and `.write_safe()`.
   - Under `panic = "abort"`, panicking across COM `extern "system"` trampolines is undefined behavior (UB). All callbacks must handle errors gracefully without unwinding.
3. **Decoupled Process Sampling**:
   - Heavy Win32 process queries (`CreateToolhelp32Snapshot`, `GetProcessMemoryInfo`, `QueryFullProcessImageNameW`) must remain decoupled to a **1.0-second interval**.
   - The physical network throughput polling loop (`GetIfTable2`) runs at **500ms**, applying fresh rate scaling without re-sampling processes on every sub-second tick.
4. **No Raw Emojis in Adaptive Cards**:
   - Windows Widgets Board renders emoji fonts inconsistently across display scaling factors and themes.
   - All icons and badges in cards must use pre-rasterized vector PNG data URIs defined in [`crates/core/src/icons.rs`](crates/core/src/icons.rs).
5. **Bounded Caches & Memory Protection**:
   - Static caches (such as `ICON_CACHE` in `process.rs`) must be capped (maximum 256 entries) to prevent unbounded memory growth during multi-day sessions.
6. **Single Source of Truth for Versions**:
   - The project version is defined strictly in [`Cargo.toml`](Cargo.toml). [`scripts/setup_msix.ps1`](scripts/setup_msix.ps1) and CI workflows automatically propagate this version to [`Package.appxmanifest`](widget/Package.appxmanifest).
7. **100% Offline & Private**:
   - Do not add outbound network telemetry, analytics, or external tracking libraries. All telemetry processing must remain entirely local.

---

## 📦 Packaging Workflows

- **Local Sideloading (`scripts/setup_msix.ps1`)**:
  Builds, signs with a local developer test certificate (`CN=NetFlow-Dev-Test`), and registers the package on your machine for live testing in the Windows 11 Widgets Board.
- **Automated CI/CD Releases (`.github/workflows/publish.yml`)**:
  Triggered by release tags (`vX.Y.Z`). Automatically runs tests, builds optimized binaries with LTO, creates GitHub Releases, and uploads to the Microsoft Store via GitHub repository secrets.

---

## 📝 Pull Request Checklist

Before submitting your pull request:
- [ ] Added unit tests covering new logic or bug fixes.
- [ ] Verified that `cargo test --workspace` passes cleanly (all 75 tests).
- [ ] Verified that `cargo clippy --workspace --all-targets --all-features -- -D warnings` returns 0 warnings.
- [ ] Verified live widget functionality via `scripts/setup_msix.ps1`.
- [ ] Ensured all mutex and lock accesses use `.lock_safe()` / `.read_safe()`.
- [ ] Updated documentation or `README.md` if user-facing behavior changed.

---

## 📬 Need Help?

Have questions or want to discuss an idea before implementing it? Feel free to open a [GitHub Issue](https://github.com/rockerrishabh/net-flow/issues) or reach out directly to Rishabh Kumar at [admin@rockerrishabh.me](mailto:admin@rockerrishabh.me).
