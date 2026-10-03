//! Diagnostic telemetry export controller, atomic file persistence, and safe token registry.
//!
//! Provides atomic file replacement using Windows Win32 `MoveFileExW`, an in-memory bounded
//! token registry for secure notification activation without exposing arbitrary filesystem paths,
//! and the native Win32 `GetSaveFileNameW` dialog integration.

use std::collections::VecDeque;
use std::io::Write;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

struct ExportTokenEntry {
    token: String,
    path: PathBuf,
    created_at: Instant,
}

static EXPORT_REGISTRY: Mutex<VecDeque<ExportTokenEntry>> = Mutex::new(VecDeque::new());
const MAX_REGISTRY_ENTRIES: usize = 16;
const TOKEN_TTL: Duration = Duration::from_secs(3600);

/// Registers a canonical file path into the bounded memory registry, returning an opaque token.
///
/// Evicts the oldest token once the capacity of 16 is reached.
pub fn register_export_token(path: PathBuf) -> String {
    let mut reg = EXPORT_REGISTRY.lock().unwrap();
    if reg.len() >= MAX_REGISTRY_ENTRIES {
        reg.pop_front();
    }
    let now = Instant::now();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let token = format!("{:016x}{:016x}", nanos, reg.len());
    reg.push_back(ExportTokenEntry {
        token: token.clone(),
        path,
        created_at: now,
    });
    token
}

/// Resolves an opaque token to a verified canonical filesystem path.
///
/// Ensures the token exists, is not expired, points to an existing file, and
/// rejects directories or arbitrary path injection attempts.
pub fn resolve_export_token(token: &str) -> Option<PathBuf> {
    let reg = EXPORT_REGISTRY.lock().unwrap();
    let now = Instant::now();
    for entry in reg.iter().rev() {
        if entry.token == token {
            if now.saturating_duration_since(entry.created_at) > TOKEN_TTL {
                return None;
            }
            if let Ok(canonical) = std::fs::canonicalize(&entry.path)
                && canonical.is_file()
            {
                return Some(canonical);
            }

            return None;
        }
    }
    None
}

/// Safely opens the file associated with the opaque token using Windows `ShellExecuteW`.
///
/// Rejects unverified, non-existent, or expired tokens.
pub fn open_exported_file(token: &str) -> bool {
    if let Some(path) = resolve_export_token(token) {
        use windows::Win32::UI::Shell::ShellExecuteW;
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
        use windows::core::HSTRING;

        let path_hstring = HSTRING::from(path.as_os_str());
        let open_verb = windows::core::w!("open");
        let res = unsafe {
            ShellExecuteW(
                None,
                open_verb,
                windows::core::PCWSTR(path_hstring.as_ptr()),
                None,
                None,
                SW_SHOWNORMAL,
            )
        };
        (res.0 as usize) > 32
    } else {
        false
    }
}

/// Atomically writes content to the target file.
///
/// Writes content to a hidden temporary file in the same parent directory, flushes and
/// syncs to physical media, and executes an atomic replacement via `MoveFileExW`
/// with `MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH`.
pub fn atomic_write_file(path: &Path, content: &str) -> std::io::Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;

    let uuid_suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_path = parent.join(format!(".netflow_export_{}.tmp", uuid_suffix));

    {
        let mut file = std::fs::File::create(&tmp_path)?;
        file.write_all(content.as_bytes())?;
        file.flush()?;
        file.sync_all()?;
    }

    if let Err(e) = move_file_atomic(&tmp_path, path)
        && let Err(rename_err) = std::fs::rename(&tmp_path, path)
    {
        let _ = std::fs::remove_file(&tmp_path);
        return Err(std::io::Error::other(format!(
            "Failed atomic replacement: MoveFileExW ({:?}), rename ({:?})",
            e, rename_err
        )));
    }

    Ok(())
}

fn move_file_atomic(source: &Path, destination: &Path) -> std::io::Result<()> {
    use windows::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };
    use windows::core::HSTRING;

    let src_hstring = HSTRING::from(source.as_os_str());
    let dst_hstring = HSTRING::from(destination.as_os_str());

    let flags = MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH;
    unsafe {
        let res = MoveFileExW(
            windows::core::PCWSTR(src_hstring.as_ptr()),
            windows::core::PCWSTR(dst_hstring.as_ptr()),
            flags,
        );
        if res.is_ok() {
            Ok(())
        } else {
            Err(std::io::Error::last_os_error())
        }
    }
}

/// Locates the user's default Downloads directory.
pub fn get_default_downloads_folder() -> Option<PathBuf> {
    if let Ok(profile) = std::env::var("USERPROFILE") {
        let downloads = PathBuf::from(profile).join("Downloads");
        if downloads.exists() {
            return Some(downloads);
        }
    }
    None
}

/// Generates a standardized default filename using the current local calendar date.
pub fn generate_default_filename(extension: &str) -> String {
    let now = net_flow_core::budget::current_local_ymd();
    format!(
        "netflow_diagnostics_{:04}{:02}{:02}.{}",
        now.0, now.1, now.2, extension
    )
}

/// Launches the native Win32 Save As dialog.
pub fn prompt_save_dialog(
    hwnd: windows::Win32::Foundation::HWND,
    default_filename: &str,
    filter: &str,
    default_ext: &str,
) -> Option<PathBuf> {
    use windows::Win32::UI::Controls::Dialogs::{
        GetSaveFileNameW, OFN_OVERWRITEPROMPT, OFN_PATHMUSTEXIST, OPENFILENAMEW,
    };

    let mut file_buf = vec![0u16; 1024];
    let def_name_u16: Vec<u16> = default_filename.encode_utf16().collect();
    for (i, &c) in def_name_u16.iter().enumerate().take(1023) {
        file_buf[i] = c;
    }

    let filter_u16: Vec<u16> = filter.encode_utf16().collect();
    let def_ext_u16: Vec<u16> = default_ext
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    let initial_dir = get_default_downloads_folder();
    let initial_dir_u16: Option<Vec<u16>> = initial_dir.as_ref().map(|p| {
        p.as_os_str()
            .encode_wide()
            .chain(std::iter::once(0))
            .collect()
    });

    let mut ofn: OPENFILENAMEW = unsafe { std::mem::zeroed() };
    ofn.lStructSize = std::mem::size_of::<OPENFILENAMEW>() as u32;
    ofn.hwndOwner = hwnd;
    ofn.lpstrFilter = windows::core::PCWSTR(filter_u16.as_ptr());
    ofn.lpstrFile = windows::core::PWSTR(file_buf.as_mut_ptr());
    ofn.nMaxFile = file_buf.len() as u32;
    if let Some(ref dir_u16) = initial_dir_u16 {
        ofn.lpstrInitialDir = windows::core::PCWSTR(dir_u16.as_ptr());
    }
    ofn.lpstrDefExt = windows::core::PCWSTR(def_ext_u16.as_ptr());
    ofn.Flags = OFN_OVERWRITEPROMPT | OFN_PATHMUSTEXIST;

    let res = unsafe { GetSaveFileNameW(&mut ofn) };
    if res.as_bool() {
        let len = file_buf
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(file_buf.len());
        let path_str = String::from_utf16_lossy(&file_buf[..len]);
        Some(PathBuf::from(path_str))
    } else {
        None
    }
}

/// Target diagnostic document export formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Csv,
    Json,
}

/// Coordinates file save prompt, snapshot collection, serialization, atomic write, and notification.
pub fn handle_export_dialog(
    hwnd: windows::Win32::Foundation::HWND,
    format: ExportFormat,
    backend: &mut net_flow_core::NetworkBackend,
) {
    let (filter, ext) = match format {
        ExportFormat::Csv => ("CSV Files (*.csv)\0*.csv\0All Files (*.*)\0*.*\0\0", "csv"),
        ExportFormat::Json => (
            "JSON Files (*.json)\0*.json\0All Files (*.*)\0*.*\0\0",
            "json",
        ),
    };
    let default_name = generate_default_filename(ext);

    if let Some(target_path) = prompt_save_dialog(hwnd, &default_name, filter, ext) {
        let snapshot = backend.collect_diagnostics_snapshot();
        let content: Result<String, String> = match format {
            ExportFormat::Csv => Ok(net_flow_core::export::export_to_csv(&snapshot)),
            ExportFormat::Json => {
                net_flow_core::export::export_to_json(&snapshot, true).map_err(|e| e.to_string())
            }
        };
        match content {
            Ok(data) => {
                if let Err(e) = atomic_write_file(&target_path, &data) {
                    eprintln!("Failed to write export file: {}", e);
                } else {
                    let token = register_export_token(target_path.clone());
                    let _ = crate::toast::show_export_complete_toast(&target_path, &token);
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize export: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_write_replace() {
        let temp_dir = std::env::temp_dir().join(format!("netflow_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let test_file = temp_dir.join("test_export.csv");

        // Write initial
        atomic_write_file(&test_file, "Initial,Data\n1,2").expect("initial write");
        assert_eq!(
            std::fs::read_to_string(&test_file).unwrap(),
            "Initial,Data\n1,2"
        );

        // Atomic replace
        atomic_write_file(&test_file, "Updated,Data\n3,4").expect("replace write");
        assert_eq!(
            std::fs::read_to_string(&test_file).unwrap(),
            "Updated,Data\n3,4"
        );

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_export_token_lifecycle() {
        let temp_dir =
            std::env::temp_dir().join(format!("netflow_token_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);
        let test_file = temp_dir.join("report.json");
        std::fs::write(&test_file, "{}").unwrap();

        // 1. Register and resolve
        let token = register_export_token(test_file.clone());
        assert!(!token.is_empty());

        let resolved = resolve_export_token(&token);
        assert!(resolved.is_some());
        let resolved_path = resolved.unwrap();
        assert_eq!(resolved_path, std::fs::canonicalize(&test_file).unwrap());

        // 2. Nonexistent token returns None
        assert!(resolve_export_token("nonexistent_token_1234").is_none());

        // 3. Test capacity eviction (16 entries max)
        for i in 0..20 {
            let p = temp_dir.join(format!("report_{}.json", i));
            std::fs::write(&p, "{}").unwrap();
            let _ = register_export_token(p);
        }

        // The first token should now be evicted
        assert!(resolve_export_token(&token).is_none());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_notification_activation_path_validation() {
        let temp_dir =
            std::env::temp_dir().join(format!("netflow_invalid_test_{}", std::process::id()));
        let _ = std::fs::create_dir_all(&temp_dir);

        // Attempting to resolve a directory path should return None
        let token = register_export_token(temp_dir.clone());
        assert!(resolve_export_token(&token).is_none());

        // Non-existent file should return None
        let ghost_file = temp_dir.join("does_not_exist.json");
        let ghost_token = register_export_token(ghost_file);
        assert!(resolve_export_token(&ghost_token).is_none());

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
