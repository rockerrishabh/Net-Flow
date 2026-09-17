use image::ImageEncoder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::NetworkManagement::IpHelper::{
    GetExtendedTcpTable, GetExtendedUdpTable, MIB_TCP6TABLE_OWNER_PID, MIB_TCPTABLE_OWNER_PID,
    MIB_UDP6TABLE_OWNER_PID, MIB_UDPTABLE_OWNER_PID, TCP_TABLE_OWNER_PID_ALL, UDP_TABLE_OWNER_PID,
};
use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6};
use windows::Win32::System::Threading::{
    GetProcessIoCounters, IO_COUNTERS, OpenProcess, PROCESS_NAME_FORMAT,
    PROCESS_QUERY_LIMITED_INFORMATION, QueryFullProcessImageNameW,
};

/// Represents an active application or service utilizing network connections.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActiveAppInfo {
    pub name: String,
    pub process_name: String,
    pub icon: &'static str,
    #[serde(default)]
    pub icon_data_uri: Option<String>,
    pub connection_count: usize,
    pub rx_bps: f64,
    pub tx_bps: f64,
}

/// Split a CamelCase / PascalCase identifier into separate words so that
/// executables like `OmenCommandCenterBackground.exe` read as real product
/// names instead of one unbreakable string the widget has to cut mid-word.
pub fn split_camel_case(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::with_capacity(input.len() + 4);

    for (i, &c) in chars.iter().enumerate() {
        if i > 0 && c.is_uppercase() {
            let prev = chars[i - 1];
            let next_is_lower = chars.get(i + 1).is_some_and(|n| n.is_lowercase());
            // "fooBar" / "v2Bar" -> split; "HTTPServer" -> split before "Server"
            // but keep the acronym itself intact.
            if prev.is_lowercase()
                || prev.is_ascii_digit()
                || (prev.is_uppercase() && next_is_lower)
            {
                out.push(' ');
            }
        }
        out.push(c);
    }

    out
}

/// Helper to sanitize a raw process binary name into a clean, human-friendly Title Case name.
pub fn sanitize_process_name(raw: &str) -> String {
    let mut name = raw;
    for ext in &[".exe", ".EXE", ".dll", ".DLL", ".bin", ".BIN"] {
        if let Some(stripped) = name.strip_suffix(ext) {
            name = stripped;
            break;
        }
    }

    let arch_suffixes = &[
        "_windows_x64",
        "_windows_x86",
        "_windows_arm64",
        "_windows_arm",
        "-windows-msvc",
        "-windows-gnu",
        "-windows",
        "_win64",
        "_win32",
        "_x64",
        "_x86",
        "_amd64",
        "_arm64",
        "-x86_64",
        "-aarch64",
        "-x64",
        "-x86",
    ];

    let mut lower_name = name.to_lowercase();
    for suffix in arch_suffixes {
        if lower_name.ends_with(suffix) {
            name = &name[..name.len() - suffix.len()];
            lower_name = name.to_lowercase();
        }
    }

    let with_spaces = split_camel_case(&name.replace(['_', '-', '.'], " "));

    let words: Vec<String> = with_spaces
        .split_whitespace()
        .map(|word| {
            let lower_word = word.to_lowercase();
            match lower_word.as_str() {
                "ide" => "IDE".to_string(),
                "sdk" => "SDK".to_string(),
                "api" => "API".to_string(),
                "gui" => "GUI".to_string(),
                "cli" => "CLI".to_string(),
                "vpn" => "VPN".to_string(),
                "lan" => "LAN".to_string(),
                "wan" => "WAN".to_string(),
                "dns" => "DNS".to_string(),
                "tcp" => "TCP".to_string(),
                "udp" => "UDP".to_string(),
                "io" => "IO".to_string(),
                "ai" => "AI".to_string(),
                "ui" => "UI".to_string(),
                "db" => "DB".to_string(),
                "mqtt" => "MQTT".to_string(),
                "nv" => "NVIDIA".to_string(),
                "gpu" => "GPU".to_string(),
                "cpu" => "CPU".to_string(),
                _ => {
                    let mut chars = word.chars();
                    if let Some(first) = chars.next() {
                        format!("{}{}", first.to_uppercase(), chars.as_str())
                    } else {
                        word.to_string()
                    }
                }
            }
        })
        .collect();

    if words.is_empty() {
        raw.to_string()
    } else {
        words.join(" ")
    }
}

/// Infer a sensible emoji icon for an application based on its name keywords.
pub fn infer_app_icon(clean_name: &str) -> &'static str {
    let lower = clean_name.to_lowercase();
    if lower.contains("rust") || lower.contains("cargo") {
        "🦀"
    } else if lower.contains("python") {
        "🐍"
    } else if lower.contains("antigravity") {
        "✨"
    } else if lower.contains("code")
        || lower.contains("dev")
        || lower.contains("compiler")
        || lower.contains("studio")
        || lower.contains("lang")
        || lower.contains("git")
        || lower.contains("terminal")
        || lower.contains("node")
        || lower.contains("docker")
    {
        "💻"
    } else if lower.contains("chat")
        || lower.contains("talk")
        || lower.contains("message")
        || lower.contains("meet")
        || lower.contains("social")
        || lower.contains("discord")
        || lower.contains("slack")
        || lower.contains("teams")
    {
        "💬"
    } else if lower.contains("browser")
        || lower.contains("web")
        || lower.contains("http")
        || lower.contains("chrome")
        || lower.contains("edge")
        || lower.contains("firefox")
    {
        "🌐"
    } else if lower.contains("music")
        || lower.contains("audio")
        || lower.contains("sound")
        || lower.contains("spotify")
    {
        "🎵"
    } else if lower.contains("video")
        || lower.contains("stream")
        || lower.contains("movie")
        || lower.contains("tv")
        || lower.contains("player")
        || lower.contains("vlc")
        || lower.contains("obs")
    {
        "🎬"
    } else if lower.contains("game")
        || lower.contains("play")
        || lower.contains("steam")
        || lower.contains("epic")
        || lower.contains("riot")
        || lower.contains("valorant")
    {
        "🎮"
    } else if lower.contains("vpn")
        || lower.contains("tunnel")
        || lower.contains("guard")
        || lower.contains("sec")
        || lower.contains("shield")
    {
        "🔒"
    } else if lower.contains("cloud")
        || lower.contains("sync")
        || lower.contains("drive")
        || lower.contains("dropbox")
        || lower.contains("onedrive")
    {
        "☁️"
    } else if lower.contains("mail") || lower.contains("outlook") || lower.contains("email") {
        "✉️"
    } else if lower.contains("note")
        || lower.contains("doc")
        || lower.contains("pdf")
        || lower.contains("edit")
    {
        "📝"
    } else if lower.contains("server")
        || lower.contains("service")
        || lower.contains("host")
        || lower.contains("system")
        || lower.contains("daemon")
        || lower.contains("agent")
    {
        "⚙️"
    } else {
        "🌐"
    }
}

/// Map a raw executable file name to a user-friendly application display name and icon.
pub fn map_process_to_app(exe_name: &str) -> (String, &'static str) {
    let lower = exe_name.to_lowercase();
    match lower.as_str() {
        // Browsers
        "chrome.exe" => ("Chrome".to_string(), "🌐"),
        "msedge.exe" => ("Edge".to_string(), "🌐"),
        "firefox.exe" => ("Firefox".to_string(), "🌐"),
        "brave.exe" => ("Brave".to_string(), "🌐"),
        "opera.exe" | "opera_gx.exe" => ("Opera".to_string(), "🌐"),
        "zen.exe" => ("Zen Browser".to_string(), "🌐"),
        "vivaldi.exe" => ("Vivaldi".to_string(), "🌐"),
        "arc.exe" => ("Arc Browser".to_string(), "🌐"),
        "tor.exe" => ("Tor Browser".to_string(), "🧅"),

        // Communication & Social
        "discord.exe" => ("Discord".to_string(), "💬"),
        "slack.exe" => ("Slack".to_string(), "💬"),
        "telegram.exe" => ("Telegram".to_string(), "💬"),
        "whatsapp.exe" => ("WhatsApp".to_string(), "💬"),
        "teams.exe" | "ms-teams.exe" => ("Microsoft Teams".to_string(), "💬"),
        "zoom.exe" => ("Zoom".to_string(), "📹"),
        "skype.exe" => ("Skype".to_string(), "💬"),
        "signal.exe" => ("Signal".to_string(), "💬"),

        // Media & Streaming
        "spotify.exe" => ("Spotify".to_string(), "🎵"),
        "applemusic.exe" => ("Apple Music".to_string(), "🎵"),
        "vlc.exe" => ("VLC Media Player".to_string(), "🎬"),
        "obs64.exe" | "obs.exe" => ("OBS Studio".to_string(), "🎥"),
        "netflix.exe" => ("Netflix".to_string(), "🎬"),

        // Gaming & Launchers
        "steam.exe" | "steamservice.exe" | "steamwebhelper.exe" => ("Steam".to_string(), "🎮"),
        "epicgameslauncher.exe" => ("Epic Games".to_string(), "🎮"),
        "battlenet.exe" | "battle.net.exe" => ("Battle.net".to_string(), "🎮"),
        "riotclientservices.exe" | "leagueclient.exe" | "valorant.exe" => {
            ("Riot Games".to_string(), "🎮")
        }
        "eadesktop.exe" | "ea.exe" | "origin.exe" => ("EA App".to_string(), "🎮"),
        "gog_galaxy.exe" => ("GOG Galaxy".to_string(), "🎮"),

        // Development & IDEs
        "code.exe" => ("VS Code".to_string(), "💻"),
        "cursor.exe" => ("Cursor".to_string(), "💻"),
        "antigravity.exe" => ("Antigravity".to_string(), "✨"),
        "rust-analyzer.exe" => ("Rust Analyzer".to_string(), "🦀"),
        "language_server_windows_x64.exe" | "language_server_windows_arm64.exe" => {
            ("Language Server".to_string(), "💻")
        }
        "devenv.exe" => ("Visual Studio".to_string(), "💻"),
        "idea64.exe" | "pycharm64.exe" | "clion64.exe" | "webstorm64.exe" | "rider64.exe"
        | "goland64.exe" | "datagrip64.exe" => ("JetBrains IDE".to_string(), "💻"),
        "git.exe" | "git-remote-https.exe" => ("Git".to_string(), "💻"),
        "cargo.exe" | "rustc.exe" => ("Rust Cargo".to_string(), "🦀"),
        "node.exe" => ("Node.js".to_string(), "💻"),
        "deno.exe" => ("Deno".to_string(), "💻"),
        "bun.exe" => ("Bun".to_string(), "💻"),
        "python.exe" | "pythonw.exe" => ("Python".to_string(), "🐍"),
        "docker.exe" | "dockerd.exe" => ("Docker".to_string(), "🐳"),
        "sublime_text.exe" => ("Sublime Text".to_string(), "💻"),
        "notepad++.exe" => ("Notepad++".to_string(), "📝"),
        "powershell.exe" | "pwsh.exe" | "cmd.exe" | "windowsterminal.exe" | "openconsole.exe" => {
            ("Terminal".to_string(), "💻")
        }

        // VPN & Networking
        "wireguard.exe" => ("WireGuard".to_string(), "🔒"),
        "tailscale.exe" | "tailscaled.exe" => ("Tailscale".to_string(), "🔒"),
        "openvpn.exe" => ("OpenVPN".to_string(), "🔒"),

        // Cloud & System
        "onedrive.exe" => ("Microsoft OneDrive".to_string(), "☁️"),
        "dropbox.exe" => ("Dropbox".to_string(), "☁️"),
        "googledrivefs.exe" => ("Google Drive".to_string(), "☁️"),
        "svchost.exe" => ("Host Process (svchost)".to_string(), "⚙️"),
        "searchhost.exe" | "startmenuexperiencehost.exe" => ("Windows Search".to_string(), "⚙️"),
        "widgets.exe" | "widgetservice.exe" => ("Windows Widgets".to_string(), "📊"),
        "explorer.exe" => ("Windows Explorer".to_string(), "📁"),
        "system" => ("Windows Kernel".to_string(), "⚙️"),

        // Hardware, GPU & Device Utilities
        "nvcontainer.exe" | "nvdisplay.container.exe" => ("NVIDIA Container".to_string(), "⚙️"),
        "nvcplui.exe" => ("NVIDIA Control Panel".to_string(), "⚙️"),
        "geforcenow.exe" => ("GeForce NOW".to_string(), "🎮"),
        "geforceexperience.exe" => ("GeForce Experience".to_string(), "🎮"),
        "omenmqtt.exe"
        | "omencommandcenterbackground.exe"
        | "omencommandcenter.exe"
        | "hp.omengaminghub.exe" => ("OMEN Gaming Hub".to_string(), "🎮"),

        // Download Managers & File Transfer
        "idman.exe" | "iemonitor.exe" | "idmbroker.exe" => {
            ("Internet Download Manager".to_string(), "🌐")
        }
        "fdm.exe" => ("Free Download Manager".to_string(), "🌐"),
        "jdownloader.exe" | "jdownloader2.exe" => ("JDownloader".to_string(), "🌐"),
        "qbittorrent.exe" => ("qBittorrent".to_string(), "📥"),
        "utorrent.exe" => ("µTorrent".to_string(), "📥"),
        "transmission.exe" | "transmission-qt.exe" => ("Transmission".to_string(), "📥"),
        "aria2c.exe" => ("aria2".to_string(), "📥"),

        // Fallback: smart sanitizer + icon inference
        _ => {
            let clean = sanitize_process_name(exe_name);
            let icon = infer_app_icon(&clean);
            (clean, icon)
        }
    }
}

#[allow(
    non_snake_case,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms
)]
#[repr(C)]
struct SHFILEINFOW {
    hIcon: isize,
    iIcon: i32,
    dwAttributes: u32,
    szDisplayName: [u16; 260],
    szTypeName: [u16; 80],
}

const SHGFI_ICON: u32 = 0x000000100;
const SHGFI_SMALLICON: u32 = 0x000000001;

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct ICONINFO {
    fIcon: i32,
    xHotspot: u32,
    yHotspot: u32,
    hbmMask: isize,
    hbmColor: isize,
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct BITMAP {
    bmType: i32,
    bmWidth: i32,
    bmHeight: i32,
    bmWidthBytes: i32,
    bmPlanes: u16,
    bmBitsPixel: u16,
    bmBits: *mut u8,
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct BITMAPINFOHEADER {
    biSize: u32,
    biWidth: i32,
    biHeight: i32,
    biPlanes: u16,
    biBitCount: u16,
    biCompression: u32,
    biSizeImage: u32,
    biXPelsPerMeter: i32,
    biYPelsPerMeter: i32,
    biClrUsed: u32,
    biClrImportant: u32,
}

#[allow(non_snake_case, dead_code, clippy::upper_case_acronyms)]
#[repr(C)]
struct BITMAPINFO {
    bmiHeader: BITMAPINFOHEADER,
    bmiColors: [u32; 1],
}

#[link(name = "shell32")]
unsafe extern "system" {
    fn SHGetFileInfoW(
        pszPath: *const u16,
        dwFileAttributes: u32,
        psfi: *mut SHFILEINFOW,
        cbFileInfo: u32,
        uFlags: u32,
    ) -> usize;
    fn ExtractIconExW(
        lpszFile: *const u16,
        nIconIndex: i32,
        phiconLarge: *mut isize,
        phiconSmall: *mut isize,
        nIcons: u32,
    ) -> u32;
}

#[link(name = "user32")]
unsafe extern "system" {
    fn DestroyIcon(hIcon: isize) -> i32;
    fn GetIconInfo(hIcon: isize, piconinfo: *mut ICONINFO) -> i32;
    fn GetDC(hWnd: isize) -> isize;
    fn ReleaseDC(hWnd: isize, hDC: isize) -> i32;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn GetObjectW(hgdiobj: isize, cbBuffer: i32, lpvObject: *mut core::ffi::c_void) -> i32;
    fn GetDIBits(
        hdc: isize,
        hbm: isize,
        start: u32,
        cLines: u32,
        lpvBits: *mut u8,
        lpbmi: *mut BITMAPINFO,
        usage: u32,
    ) -> i32;
    fn DeleteObject(ho: isize) -> i32;
}

static ICON_CACHE: std::sync::LazyLock<std::sync::Mutex<HashMap<String, Option<String>>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(HashMap::new()));

/// Extract the native Windows icon from an executable file as a base64 PNG data URI.
pub fn get_process_native_icon(full_path: &str) -> Option<String> {
    if full_path.is_empty() || full_path == "System" {
        return None;
    }

    {
        let cache = ICON_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(cached) = cache.get(full_path) {
            return cached.clone();
        }
    }

    let icon_data_uri = extract_native_icon_data_uri(full_path);

    {
        let mut cache = ICON_CACHE.lock().unwrap_or_else(|e| e.into_inner());
        if cache.len() >= 256 {
            cache.clear();
        }
        cache.insert(full_path.to_string(), icon_data_uri.clone());
    }

    icon_data_uri
}

fn extract_native_icon_data_uri(path: &str) -> Option<String> {
    unsafe {
        let wide: Vec<u16> = path.encode_utf16().chain(std::iter::once(0)).collect();
        let mut hicon_small: isize = 0;
        let count = ExtractIconExW(wide.as_ptr(), 0, std::ptr::null_mut(), &mut hicon_small, 1);
        let hicon = if count > 0 && hicon_small != 0 {
            hicon_small
        } else {
            let mut shfi = SHFILEINFOW {
                hIcon: 0,
                iIcon: 0,
                dwAttributes: 0,
                szDisplayName: [0; 260],
                szTypeName: [0; 80],
            };
            let res = SHGetFileInfoW(
                wide.as_ptr(),
                0,
                &mut shfi,
                std::mem::size_of::<SHFILEINFOW>() as u32,
                SHGFI_ICON | SHGFI_SMALLICON,
            );
            if res != 0 && shfi.hIcon != 0 {
                shfi.hIcon
            } else {
                return None;
            }
        };

        let mut icon_info = ICONINFO {
            fIcon: 0,
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: 0,
            hbmColor: 0,
        };
        if GetIconInfo(hicon, &mut icon_info) == 0 {
            DestroyIcon(hicon);
            return None;
        }

        let mut bmp = BITMAP {
            bmType: 0,
            bmWidth: 0,
            bmHeight: 0,
            bmWidthBytes: 0,
            bmPlanes: 0,
            bmBitsPixel: 0,
            bmBits: std::ptr::null_mut(),
        };
        let get_obj_res = GetObjectW(
            icon_info.hbmColor,
            std::mem::size_of::<BITMAP>() as i32,
            &mut bmp as *mut _ as *mut _,
        );

        if get_obj_res == 0 || bmp.bmWidth <= 0 || bmp.bmHeight <= 0 {
            if icon_info.hbmColor != 0 {
                DeleteObject(icon_info.hbmColor);
            }
            if icon_info.hbmMask != 0 {
                DeleteObject(icon_info.hbmMask);
            }
            DestroyIcon(hicon);
            return None;
        }

        let w = bmp.bmWidth as u32;
        let h = bmp.bmHeight as u32;

        let hdc = GetDC(0);
        if hdc == 0 {
            if icon_info.hbmColor != 0 {
                DeleteObject(icon_info.hbmColor);
            }
            if icon_info.hbmMask != 0 {
                DeleteObject(icon_info.hbmMask);
            }
            DestroyIcon(hicon);
            return None;
        }

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w as i32,
                biHeight: -(h as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: 0,
                biXPelsPerMeter: 0,
                biYPelsPerMeter: 0,
                biClrUsed: 0,
                biClrImportant: 0,
            },
            bmiColors: [0],
        };

        let mut bgra_buf = vec![0u8; (w * h * 4) as usize];
        let lines = GetDIBits(
            hdc,
            icon_info.hbmColor,
            0,
            h,
            bgra_buf.as_mut_ptr(),
            &mut bmi,
            0,
        );

        ReleaseDC(0, hdc);
        if icon_info.hbmColor != 0 {
            DeleteObject(icon_info.hbmColor);
        }
        if icon_info.hbmMask != 0 {
            DeleteObject(icon_info.hbmMask);
        }
        DestroyIcon(hicon);

        if lines == 0 {
            return None;
        }

        // Convert BGRA to RGBA & check alpha
        let mut has_non_zero_alpha = false;
        for chunk in bgra_buf.as_chunks_mut::<4>().0 {
            let b = chunk[0];
            let r = chunk[2];
            let a = chunk[3];
            chunk[0] = r;
            chunk[2] = b;
            if a > 0 {
                has_non_zero_alpha = true;
            }
        }
        if !has_non_zero_alpha {
            for chunk in bgra_buf.as_chunks_mut::<4>().0 {
                chunk[3] = 255;
            }
        }

        let mut png_bytes = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        if encoder
            .write_image(&bgra_buf, w, h, image::ExtendedColorType::Rgba8)
            .is_ok()
        {
            Some(crate::chart::png_to_data_uri(&png_bytes))
        } else {
            None
        }
    }
}

/// Query the process full executable path, name and I/O byte transfer counters for a given process ID.
pub fn get_process_info_and_io(pid: u32) -> Option<(String, String, u64, u64)> {
    if pid == 0 {
        return None;
    }
    if pid == 4 {
        return Some(("System".to_string(), "System".to_string(), 0, 0));
    }

    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid).ok()?;
        let mut buf = [0u16; 1024];
        let mut size = buf.len() as u32;
        let res = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_FORMAT(0),
            windows::core::PWSTR(buf.as_mut_ptr()),
            &mut size,
        );
        let mut io = IO_COUNTERS::default();
        let io_res = GetProcessIoCounters(handle, &mut io);
        let _ = CloseHandle(handle);

        if res.is_ok() && size > 0 {
            let full_path = String::from_utf16_lossy(&buf[..size as usize]);
            let exe_name = full_path
                .split('\\')
                .next_back()
                .unwrap_or(&full_path)
                .to_string();
            let (read_bytes, write_bytes) = if io_res.is_ok() {
                (io.ReadTransferCount, io.WriteTransferCount)
            } else {
                (0, 0)
            };
            Some((full_path, exe_name, read_bytes, write_bytes))
        } else {
            None
        }
    }
}

/// Query all active socket connections (TCP IPv4/IPv6 and UDP IPv4/IPv6) and count per PID.
pub fn query_socket_pids() -> (HashMap<u32, usize>, usize) {
    let mut pid_counts: HashMap<u32, usize> = HashMap::new();
    let mut total_connections = 0usize;

    // 1. IPv4 TCP
    let mut size4_tcp = 0u32;
    let _ = unsafe {
        GetExtendedTcpTable(
            None,
            &mut size4_tcp,
            false,
            AF_INET.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };
    if size4_tcp > 0 {
        let mut buf = vec![0u8; size4_tcp as usize];
        let res = unsafe {
            GetExtendedTcpTable(
                Some(buf.as_mut_ptr() as *mut _),
                &mut size4_tcp,
                false,
                AF_INET.0 as u32,
                TCP_TABLE_OWNER_PID_ALL,
                0,
            )
        };
        if res == 0 {
            let table = unsafe { &*(buf.as_ptr() as *const MIB_TCPTABLE_OWNER_PID) };
            let entries = unsafe {
                std::slice::from_raw_parts(table.table.as_ptr(), table.dwNumEntries as usize)
            };
            for entry in entries {
                if entry.dwOwningPid > 0 {
                    *pid_counts.entry(entry.dwOwningPid).or_insert(0) += 1;
                    total_connections += 1;
                }
            }
        }
    }

    // 2. IPv6 TCP
    let mut size6_tcp = 0u32;
    let _ = unsafe {
        GetExtendedTcpTable(
            None,
            &mut size6_tcp,
            false,
            AF_INET6.0 as u32,
            TCP_TABLE_OWNER_PID_ALL,
            0,
        )
    };
    if size6_tcp > 0 {
        let mut buf = vec![0u8; size6_tcp as usize];
        let res = unsafe {
            GetExtendedTcpTable(
                Some(buf.as_mut_ptr() as *mut _),
                &mut size6_tcp,
                false,
                AF_INET6.0 as u32,
                TCP_TABLE_OWNER_PID_ALL,
                0,
            )
        };
        if res == 0 {
            let table = unsafe { &*(buf.as_ptr() as *const MIB_TCP6TABLE_OWNER_PID) };
            let entries = unsafe {
                std::slice::from_raw_parts(table.table.as_ptr(), table.dwNumEntries as usize)
            };
            for entry in entries {
                if entry.dwOwningPid > 0 {
                    *pid_counts.entry(entry.dwOwningPid).or_insert(0) += 1;
                    total_connections += 1;
                }
            }
        }
    }

    // 3. IPv4 UDP
    let mut size4_udp = 0u32;
    let _ = unsafe {
        GetExtendedUdpTable(
            None,
            &mut size4_udp,
            false,
            AF_INET.0 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        )
    };
    if size4_udp > 0 {
        let mut buf = vec![0u8; size4_udp as usize];
        let res = unsafe {
            GetExtendedUdpTable(
                Some(buf.as_mut_ptr() as *mut _),
                &mut size4_udp,
                false,
                AF_INET.0 as u32,
                UDP_TABLE_OWNER_PID,
                0,
            )
        };
        if res == 0 {
            let table = unsafe { &*(buf.as_ptr() as *const MIB_UDPTABLE_OWNER_PID) };
            let entries = unsafe {
                std::slice::from_raw_parts(table.table.as_ptr(), table.dwNumEntries as usize)
            };
            for entry in entries {
                if entry.dwOwningPid > 0 {
                    *pid_counts.entry(entry.dwOwningPid).or_insert(0) += 1;
                    total_connections += 1;
                }
            }
        }
    }

    // 4. IPv6 UDP
    let mut size6_udp = 0u32;
    let _ = unsafe {
        GetExtendedUdpTable(
            None,
            &mut size6_udp,
            false,
            AF_INET6.0 as u32,
            UDP_TABLE_OWNER_PID,
            0,
        )
    };
    if size6_udp > 0 {
        let mut buf = vec![0u8; size6_udp as usize];
        let res = unsafe {
            GetExtendedUdpTable(
                Some(buf.as_mut_ptr() as *mut _),
                &mut size6_udp,
                false,
                AF_INET6.0 as u32,
                UDP_TABLE_OWNER_PID,
                0,
            )
        };
        if res == 0 {
            let table = unsafe { &*(buf.as_ptr() as *const MIB_UDP6TABLE_OWNER_PID) };
            let entries = unsafe {
                std::slice::from_raw_parts(table.table.as_ptr(), table.dwNumEntries as usize)
            };
            for entry in entries {
                if entry.dwOwningPid > 0 {
                    *pid_counts.entry(entry.dwOwningPid).or_insert(0) += 1;
                    total_connections += 1;
                }
            }
        }
    }

    (pid_counts, total_connections)
}

/// Tracks process I/O counters over time to calculate per-process realtime upload and download rates.
#[derive(Debug, Default, Clone)]
pub struct ProcessTracker {
    prev_io: HashMap<u32, (u64, u64)>, // pid -> (read_transfer_count, write_transfer_count)
    prev_time: Option<std::time::Instant>,
}

impl ProcessTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.prev_io.clear();
        self.prev_time = None;
    }

    pub fn sample(&mut self, now: std::time::Instant) -> (Vec<ActiveAppInfo>, usize) {
        let elapsed_secs = match self.prev_time {
            Some(prev) => {
                let duration = now.saturating_duration_since(prev).as_secs_f64();
                if duration > 0.0 { duration } else { 0.0 }
            }
            None => 0.0,
        };
        self.prev_time = Some(now);

        let (pid_conns, total_connections) = query_socket_pids();

        let mut new_io_map = HashMap::new();
        let mut app_map: HashMap<String, ActiveAppInfo> = HashMap::new();

        for (pid, conns) in pid_conns {
            if let Some((full_path, exe_name, curr_read, curr_write)) = get_process_info_and_io(pid)
            {
                let (delta_read, delta_write) = match self.prev_io.get(&pid) {
                    Some(&(prev_read, prev_write)) => (
                        curr_read.saturating_sub(prev_read),
                        curr_write.saturating_sub(prev_write),
                    ),
                    None => (0, 0),
                };

                new_io_map.insert(pid, (curr_read, curr_write));

                let rx_bps = if elapsed_secs > 0.0 {
                    delta_read as f64 / elapsed_secs
                } else {
                    0.0
                };
                let tx_bps = if elapsed_secs > 0.0 {
                    delta_write as f64 / elapsed_secs
                } else {
                    0.0
                };

                let (app_name, icon) = map_process_to_app(&exe_name);
                let native_icon = get_process_native_icon(&full_path);
                let entry = app_map
                    .entry(app_name.clone())
                    .or_insert_with(|| ActiveAppInfo {
                        name: app_name,
                        process_name: exe_name,
                        icon,
                        icon_data_uri: native_icon.clone(),
                        connection_count: 0,
                        rx_bps: 0.0,
                        tx_bps: 0.0,
                    });
                if entry.icon_data_uri.is_none() && native_icon.is_some() {
                    entry.icon_data_uri = native_icon;
                }
                entry.connection_count += conns;
                entry.rx_bps += rx_bps;
                entry.tx_bps += tx_bps;
            }
        }

        self.prev_io = new_io_map;

        let mut apps: Vec<ActiveAppInfo> = app_map.into_values().collect();
        apps.sort_by(compare_active_apps);

        (apps, total_connections)
    }
}

/// Query all active socket connections grouped by active applications in real time (stateless fallback).
pub fn query_active_apps() -> (Vec<ActiveAppInfo>, usize) {
    let mut tracker = ProcessTracker::new();
    tracker.sample(std::time::Instant::now())
}

/// Deterministic and stable ordering for active apps:
/// - Apps with active bandwidth (>= 1.0 B/s) rank above idle apps (< 1.0 B/s).
/// - Active apps are sorted primarily by bandwidth descending, then by connection count.
/// - Idle apps are sorted strictly by connection count descending, then alphabetically by name.
///
/// This prevents sub-byte I/O noise from causing the active apps list to shuffle when idle.
pub fn compare_active_apps(a: &ActiveAppInfo, b: &ActiveAppInfo) -> std::cmp::Ordering {
    let rate_a = a.rx_bps + a.tx_bps;
    let rate_b = b.rx_bps + b.tx_bps;
    let a_active = rate_a >= 1.0;
    let b_active = rate_b >= 1.0;

    match (a_active, b_active) {
        (true, true) => rate_b
            .partial_cmp(&rate_a)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.connection_count.cmp(&a.connection_count))
            .then_with(|| a.name.cmp(&b.name)),
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        (false, false) => b
            .connection_count
            .cmp(&a.connection_count)
            .then_with(|| a.name.cmp(&b.name)),
    }
}

/// Reconcile per-process I/O rates with the authoritative physical network throughput.
///
/// Process I/O counters on Windows (`GetProcessIoCounters`) aggregate all I/O, including
/// file reads/writes, pipes, and disk cache. If total network traffic is zero or low,
/// disk activity must not be reported as internet bandwidth.
fn is_system_app(app: &ActiveAppInfo) -> bool {
    let lower_proc = app.process_name.to_ascii_lowercase();
    let lower_name = app.name.to_ascii_lowercase();
    lower_proc == "system"
        || lower_proc == "svchost.exe"
        || lower_proc == "registry"
        || lower_proc == "smss.exe"
        || lower_proc == "csrss.exe"
        || lower_proc == "services.exe"
        || lower_proc == "lsass.exe"
        || lower_name.contains("kernel")
        || lower_name.contains("host process")
}

pub fn reconcile_app_bandwidth(
    active_apps: &mut [ActiveAppInfo],
    net_rx_bps: f64,
    net_tx_bps: f64,
) {
    if active_apps.is_empty() {
        return;
    }

    if net_rx_bps <= 0.0 && net_tx_bps <= 0.0 {
        for app in active_apps.iter_mut() {
            app.rx_bps = 0.0;
            app.tx_bps = 0.0;
        }
        return;
    }

    // Directional I/O reconciliation:
    // When an app downloads files from the internet (e.g. IDM, Steam, browsers, torrents),
    // it writes incoming chunks to disk (`WriteFile`), incrementing `WriteTransferCount`.
    // Winsock socket receives (`WSARecv`) do not increment `ReadTransferCount`.
    // Conversely, when an app uploads files (e.g. OneDrive, cloud sync, web uploads),
    // it reads data from disk (`ReadFile`), incrementing `ReadTransferCount`.
    //
    // Naively mapping `ReadTransferCount` -> `rx_bps` and `WriteTransferCount` -> `tx_bps`
    // causes file downloads to appear as uploads, and file uploads to appear as downloads.
    //
    // We detect impossible directional mismatches against physical adapter throughput:
    if net_rx_bps > 2.0 * net_tx_bps && net_rx_bps > 10_000.0 {
        let mut download_shifted = false;
        for app in active_apps.iter_mut() {
            if app.tx_bps > net_tx_bps && app.rx_bps < net_tx_bps {
                app.rx_bps = app.tx_bps.min(net_rx_bps);
                app.tx_bps = 0.0;
                download_shifted = true;
            }
        }
        if download_shifted {
            let total_rx: f64 = active_apps.iter().map(|a| a.rx_bps).sum();
            if total_rx > 0.0 {
                for app in active_apps.iter_mut() {
                    if app.rx_bps > 0.0 {
                        app.tx_bps = (app.rx_bps / total_rx) * net_tx_bps;
                    }
                }
            }
        }
    } else if net_tx_bps > 2.0 * net_rx_bps && net_tx_bps > 10_000.0 {
        let mut upload_shifted = false;
        for app in active_apps.iter_mut() {
            if app.rx_bps > net_rx_bps && app.tx_bps < net_rx_bps {
                app.tx_bps = app.rx_bps.min(net_tx_bps);
                app.rx_bps = 0.0;
                upload_shifted = true;
            }
        }
        if upload_shifted {
            let total_tx: f64 = active_apps.iter().map(|a| a.tx_bps).sum();
            if total_tx > 0.0 {
                for app in active_apps.iter_mut() {
                    if app.tx_bps > 0.0 {
                        app.rx_bps = (app.tx_bps / total_tx) * net_rx_bps;
                    }
                }
            }
        }
    }

    if net_rx_bps <= 0.0 {
        for app in active_apps.iter_mut() {
            app.rx_bps = 0.0;
        }
    } else {
        let total_app_rx: f64 = active_apps.iter().map(|a| a.rx_bps).sum();
        if total_app_rx > net_rx_bps && total_app_rx > 0.0 {
            let scale = net_rx_bps / total_app_rx;
            for app in active_apps.iter_mut() {
                app.rx_bps *= scale;
            }
        }
    }

    if net_tx_bps <= 0.0 {
        for app in active_apps.iter_mut() {
            app.tx_bps = 0.0;
        }
    } else {
        let total_app_tx: f64 = active_apps.iter().map(|a| a.tx_bps).sum();
        if total_app_tx > net_tx_bps && total_app_tx > 0.0 {
            let scale = net_tx_bps / total_app_tx;
            for app in active_apps.iter_mut() {
                app.tx_bps *= scale;
            }
        }
    }

    // Unmeasured socket throughput pro-rating:
    // When the physical network adapter is active but individual processes did not
    // produce Win32 file I/O (e.g. lightweight socket streams in Chrome, Antigravity, etc.),
    // distribute the unmeasured bandwidth across socket holders that currently have 0 rate,
    // prioritizing non-system user apps.
    if net_rx_bps > 0.0 {
        let cur_app_rx: f64 = active_apps.iter().map(|a| a.rx_bps).sum();
        if net_rx_bps > cur_app_rx {
            let unmeasured_rx = net_rx_bps - cur_app_rx;
            let user_zero_conns: usize = active_apps
                .iter()
                .filter(|a| !is_system_app(a) && a.rx_bps <= 0.0)
                .map(|a| a.connection_count)
                .sum();

            if user_zero_conns > 0 {
                for app in active_apps.iter_mut() {
                    if !is_system_app(app) && app.rx_bps <= 0.0 && app.connection_count > 0 {
                        let fraction = app.connection_count as f64 / user_zero_conns as f64;
                        app.rx_bps += unmeasured_rx * fraction;
                    }
                }
            } else {
                let sys_zero_conns: usize = active_apps
                    .iter()
                    .filter(|a| a.rx_bps <= 0.0)
                    .map(|a| a.connection_count)
                    .sum();
                if sys_zero_conns > 0 {
                    for app in active_apps.iter_mut() {
                        if app.rx_bps <= 0.0 && app.connection_count > 0 {
                            let fraction = app.connection_count as f64 / sys_zero_conns as f64;
                            app.rx_bps += unmeasured_rx * fraction;
                        }
                    }
                }
            }
        }
    }

    if net_tx_bps > 0.0 {
        let cur_app_tx: f64 = active_apps.iter().map(|a| a.tx_bps).sum();
        if net_tx_bps > cur_app_tx {
            let unmeasured_tx = net_tx_bps - cur_app_tx;
            let user_zero_conns: usize = active_apps
                .iter()
                .filter(|a| !is_system_app(a) && a.tx_bps <= 0.0)
                .map(|a| a.connection_count)
                .sum();

            if user_zero_conns > 0 {
                for app in active_apps.iter_mut() {
                    if !is_system_app(app) && app.tx_bps <= 0.0 && app.connection_count > 0 {
                        let fraction = app.connection_count as f64 / user_zero_conns as f64;
                        app.tx_bps += unmeasured_tx * fraction;
                    }
                }
            } else {
                let sys_zero_conns: usize = active_apps
                    .iter()
                    .filter(|a| a.tx_bps <= 0.0)
                    .map(|a| a.connection_count)
                    .sum();
                if sys_zero_conns > 0 {
                    for app in active_apps.iter_mut() {
                        if app.tx_bps <= 0.0 && app.connection_count > 0 {
                            let fraction = app.connection_count as f64 / sys_zero_conns as f64;
                            app.tx_bps += unmeasured_tx * fraction;
                        }
                    }
                }
            }
        }
    }

    // Strict boundary checks: total app throughput must never exceed physical adapter throughput
    if net_rx_bps <= 0.0 {
        for app in active_apps.iter_mut() {
            app.rx_bps = 0.0;
        }
    } else {
        let final_rx: f64 = active_apps.iter().map(|a| a.rx_bps).sum();
        if final_rx > net_rx_bps && final_rx > 0.0 {
            let scale = net_rx_bps / final_rx;
            for app in active_apps.iter_mut() {
                app.rx_bps *= scale;
            }
        }
    }

    if net_tx_bps <= 0.0 {
        for app in active_apps.iter_mut() {
            app.tx_bps = 0.0;
        }
    } else {
        let final_tx: f64 = active_apps.iter().map(|a| a.tx_bps).sum();
        if final_tx > net_tx_bps && final_tx > 0.0 {
            let scale = net_tx_bps / final_tx;
            for app in active_apps.iter_mut() {
                app.tx_bps *= scale;
            }
        }
    }

    // Re-sort apps by reconciled bandwidth, then connection count
    active_apps.sort_by(compare_active_apps);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_process_to_app() {
        assert_eq!(
            map_process_to_app("chrome.exe"),
            ("Chrome".to_string(), "🌐")
        );
        assert_eq!(
            map_process_to_app("discord.exe"),
            ("Discord".to_string(), "💬")
        );
        assert_eq!(
            map_process_to_app("spotify.exe"),
            ("Spotify".to_string(), "🎵")
        );
        assert_eq!(
            map_process_to_app("svchost.exe"),
            ("Host Process (svchost)".to_string(), "⚙️")
        );
        assert_eq!(
            map_process_to_app("antigravity.exe"),
            ("Antigravity".to_string(), "✨")
        );
        assert_eq!(
            map_process_to_app("rust-analyzer.exe"),
            ("Rust Analyzer".to_string(), "🦀")
        );
        assert_eq!(
            map_process_to_app("language_server_windows_x64.exe"),
            ("Language Server".to_string(), "💻")
        );
    }

    #[test]
    fn test_sanitize_process_name() {
        assert_eq!(
            sanitize_process_name("my_custom_tool_windows_x64.exe"),
            "My Custom Tool"
        );
        assert_eq!(
            sanitize_process_name("backend-api-server-x86_64.exe"),
            "Backend API Server"
        );
        assert_eq!(sanitize_process_name("vpn_client_win64.exe"), "VPN Client");
    }

    #[test]
    fn test_sanitize_splits_camel_case_names() {
        assert_eq!(
            sanitize_process_name("OmenCommandCenterBackground.exe"),
            "Omen Command Center Background"
        );
        assert_eq!(sanitize_process_name("BingWallpaper.exe"), "Bing Wallpaper");
        // Acronyms stay glued to themselves.
        assert_eq!(sanitize_process_name("HTTPServer.exe"), "HTTP Server");
        // Already-spaced names are untouched.
        assert_eq!(sanitize_process_name("Notepad.exe"), "Notepad");
    }

    #[test]
    fn test_infer_app_icon() {
        assert_eq!(infer_app_icon("Rust Cargo Build"), "🦀");
        assert_eq!(infer_app_icon("Python Script"), "🐍");
        assert_eq!(infer_app_icon("VPN Client"), "🔒");
        assert_eq!(infer_app_icon("Audio Streamer"), "🎵");
    }

    #[test]
    fn test_query_active_apps_runs() {
        let (_apps, _total) = query_active_apps();
    }

    #[test]
    fn test_map_process_to_app_hardware_and_drivers() {
        assert_eq!(
            map_process_to_app("nvcontainer.exe"),
            ("NVIDIA Container".to_string(), "⚙️")
        );
        assert_eq!(
            map_process_to_app("omenmqtt.exe"),
            ("OMEN Gaming Hub".to_string(), "🎮")
        );
    }

    #[test]
    fn test_reconcile_app_bandwidth_zero_network() {
        let mut apps = vec![ActiveAppInfo {
            name: "Nvcontainer".to_string(),
            process_name: "nvcontainer.exe".to_string(),
            icon: "⚙️",
            icon_data_uri: None,
            connection_count: 5,
            rx_bps: 14600.0, // Disk read activity
            tx_bps: 153.0,
        }];
        // Zero network download must zero out disk reads
        reconcile_app_bandwidth(&mut apps, 0.0, 200.0);
        assert_eq!(apps[0].rx_bps, 0.0);
        assert_eq!(apps[0].tx_bps, 153.0);
    }

    #[test]
    fn test_reconcile_app_bandwidth_scales_to_physical_throughput() {
        let mut apps = vec![
            ActiveAppInfo {
                name: "App 1".to_string(),
                process_name: "app1.exe".to_string(),
                icon: "🌐",
                icon_data_uri: None,
                connection_count: 2,
                rx_bps: 10_000.0,
                tx_bps: 500.0,
            },
            ActiveAppInfo {
                name: "App 2".to_string(),
                process_name: "app2.exe".to_string(),
                icon: "🌐",
                icon_data_uri: None,
                connection_count: 3,
                rx_bps: 10_000.0,
                tx_bps: 500.0,
            },
        ];
        // Total app rx is 20,000, but physical adapter only received 500 B/s
        reconcile_app_bandwidth(&mut apps, 500.0, 1000.0);
        assert_eq!(apps[0].rx_bps, 250.0);
        assert_eq!(apps[1].rx_bps, 250.0);
        // tx was 1000 total, which is <= 1000 net_tx_bps, so unscaled
        assert_eq!(apps[0].tx_bps, 500.0);
        assert_eq!(apps[1].tx_bps, 500.0);
    }

    #[test]
    fn test_map_process_to_app_download_managers() {
        assert_eq!(
            map_process_to_app("idman.exe"),
            ("Internet Download Manager".to_string(), "🌐")
        );
        assert_eq!(
            map_process_to_app("iemonitor.exe"),
            ("Internet Download Manager".to_string(), "🌐")
        );
        assert_eq!(
            map_process_to_app("qbittorrent.exe"),
            ("qBittorrent".to_string(), "📥")
        );
        assert_eq!(
            map_process_to_app("fdm.exe"),
            ("Free Download Manager".to_string(), "🌐")
        );
    }

    #[test]
    fn test_reconcile_app_bandwidth_download_manager_disk_write() {
        let mut apps = vec![ActiveAppInfo {
            name: "Internet Download Manager".to_string(),
            process_name: "idman.exe".to_string(),
            icon: "🌐",
            icon_data_uri: None,
            connection_count: 8,
            rx_bps: 0.0,          // Winsock recv does not increment ReadTransferCount
            tx_bps: 12_280_000.0, // WriteFile to disk increments WriteTransferCount
        }];

        // Physical adapter: 12.28 MB/s download, 122 KB/s upload ACKs
        reconcile_app_bandwidth(&mut apps, 12_280_000.0, 122_000.0);

        // IDM must be attributed the 12.28 MB/s download and proportionate upload ACKs!
        assert!((apps[0].rx_bps - 12_280_000.0).abs() < 1.0);
        assert!((apps[0].tx_bps - 122_000.0).abs() < 1.0);
    }

    #[test]
    fn test_reconcile_app_bandwidth_cloud_sync_disk_read() {
        let mut apps = vec![ActiveAppInfo {
            name: "Microsoft OneDrive".to_string(),
            process_name: "onedrive.exe".to_string(),
            icon: "☁️",
            icon_data_uri: None,
            connection_count: 4,
            rx_bps: 10_000_000.0, // ReadFile from disk increments ReadTransferCount
            tx_bps: 0.0,          // Winsock send does not increment WriteTransferCount
        }];

        // Physical adapter: 100 KB/s download ACKs, 10 MB/s upload
        reconcile_app_bandwidth(&mut apps, 100_000.0, 10_000_000.0);

        // OneDrive must be attributed the 10 MB/s upload and proportionate download ACKs!
        assert!((apps[0].tx_bps - 10_000_000.0).abs() < 1.0);
        assert!((apps[0].rx_bps - 100_000.0).abs() < 1.0);
    }

    #[test]
    fn test_compare_active_apps_stabilizes_idle_order() {
        let mut apps = [
            ActiveAppInfo {
                name: "Widget Board".to_string(),
                process_name: "widgets.exe".to_string(),
                icon: "📊",
                icon_data_uri: None,
                connection_count: 1,
                rx_bps: 0.05, // Sub-byte noise
                tx_bps: 0.0,
            },
            ActiveAppInfo {
                name: "Chrome".to_string(),
                process_name: "chrome.exe".to_string(),
                icon: "🌐",
                icon_data_uri: None,
                connection_count: 35,
                rx_bps: 0.0,
                tx_bps: 0.0,
            },
            ActiveAppInfo {
                name: "Active Downloader".to_string(),
                process_name: "dl.exe".to_string(),
                icon: "📥",
                icon_data_uri: None,
                connection_count: 2,
                rx_bps: 50_000.0,
                tx_bps: 1_000.0,
            },
        ];

        apps.sort_by(compare_active_apps);

        // 1. Active app always comes first
        assert_eq!(apps[0].name, "Active Downloader");
        // 2. Idle apps are sorted by connection count (Chrome 35 > Widget Board 1), ignoring sub-byte noise
        assert_eq!(apps[1].name, "Chrome");
        assert_eq!(apps[2].name, "Widget Board");
    }

    #[test]
    fn test_reconcile_app_bandwidth_prorates_socket_holders() {
        let mut apps = vec![
            ActiveAppInfo {
                name: "Chrome".to_string(),
                process_name: "chrome.exe".to_string(),
                icon: "🌐",
                icon_data_uri: None,
                connection_count: 30,
                rx_bps: 0.0,
                tx_bps: 0.0,
            },
            ActiveAppInfo {
                name: "Antigravity".to_string(),
                process_name: "antigravity.exe".to_string(),
                icon: "💻",
                icon_data_uri: None,
                connection_count: 10,
                rx_bps: 0.0,
                tx_bps: 0.0,
            },
            ActiveAppInfo {
                name: "Windows Kernel".to_string(),
                process_name: "System".to_string(),
                icon: "⚙️",
                icon_data_uri: None,
                connection_count: 20,
                rx_bps: 0.0,
                tx_bps: 0.0,
            },
        ];

        // 1200 B/s download, 400 B/s upload
        reconcile_app_bandwidth(&mut apps, 1200.0, 400.0);

        // Chrome has 30/(30+10) = 75% of non-system sockets:
        // 75% of 1200 = 900 B/s rx, 75% of 400 = 300 B/s tx
        let chrome = apps.iter().find(|a| a.name == "Chrome").unwrap();
        assert_eq!(chrome.rx_bps, 900.0);
        assert_eq!(chrome.tx_bps, 300.0);

        // Antigravity has 10/40 = 25% of non-system sockets:
        // 25% of 1200 = 300 B/s rx, 25% of 400 = 100 B/s tx
        let antigravity = apps.iter().find(|a| a.name == "Antigravity").unwrap();
        assert_eq!(antigravity.rx_bps, 300.0);
        assert_eq!(antigravity.tx_bps, 100.0);

        // Windows Kernel is a system app, so it remains idle
        let kernel = apps.iter().find(|a| a.name == "Windows Kernel").unwrap();
        assert_eq!(kernel.rx_bps, 0.0);
        assert_eq!(kernel.tx_bps, 0.0);

        // Total matches network throughput
        let total_rx: f64 = apps.iter().map(|a| a.rx_bps).sum();
        let total_tx: f64 = apps.iter().map(|a| a.tx_bps).sum();
        assert_eq!(total_rx, 1200.0);
        assert_eq!(total_tx, 400.0);
    }
}
