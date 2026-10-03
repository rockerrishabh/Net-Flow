use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::types::{LatencySnapshot, PhysicalLinkInfo};

/// Persistent telemetry metrics saved across widget restarts and session resets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SessionState {
    #[serde(default = "default_session_generation")]
    pub generation: u64,
    #[serde(alias = "download_bytes")]
    pub session_rx: u64,
    #[serde(alias = "upload_bytes")]
    pub session_tx: u64,
    #[serde(alias = "started_at")]
    pub session_start_unix: u64,
    #[serde(default)]
    pub all_time_peak_rx: f64,
    #[serde(default)]
    pub all_time_peak_tx: f64,
    #[serde(default)]
    pub updated_at_unix: u64,
    #[serde(default)]
    pub latency: LatencySnapshot,
    #[serde(default)]
    pub physical_link: Option<PhysicalLinkInfo>,
}

const fn default_session_generation() -> u64 {
    1
}

impl Default for SessionState {
    fn default() -> Self {
        let now_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            generation: 1,
            session_rx: 0,
            session_tx: 0,
            session_start_unix: now_unix,
            all_time_peak_rx: 0.0,
            all_time_peak_tx: 0.0,
            updated_at_unix: now_unix,
            latency: LatencySnapshot::default(),
            physical_link: None,
        }
    }
}

pub fn get_session_state_path() -> PathBuf {
    if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(local_app_data)
            .join("NetFlow")
            .join("session_state.json")
    } else {
        std::env::temp_dir()
            .join("NetFlow")
            .join("session_state.json")
    }
}

pub fn load_persisted_session_state() -> SessionState {
    let path = get_session_state_path();
    if let Ok(data) = std::fs::read_to_string(&path)
        && let Ok(state) = serde_json::from_str::<SessionState>(&data)
    {
        return state;
    }
    SessionState::default()
}

/// Atomically writes content to the target file by first writing to a process-unique temporary
/// file in the same directory, syncing to disk, and renaming over the target path with retry backoff.
pub fn write_atomic(path: &Path, content: &[u8]) -> std::io::Result<()> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    std::fs::create_dir_all(parent)?;
    let file_name = path
        .file_name()
        .map(|s| s.to_string_lossy())
        .unwrap_or_else(|| "atomic".into());
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let tmp_path = parent.join(format!(
        "{}.tmp.{}.{}",
        file_name,
        std::process::id(),
        nanos
    ));

    {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&tmp_path)?;
        file.write_all(content)?;
        file.flush()?;
        file.sync_all()?;
    }

    let mut last_err = None;
    for attempt in 0..5 {
        match std::fs::rename(&tmp_path, path) {
            Ok(()) => return Ok(()),
            Err(err) => {
                last_err = Some(err);
                if attempt < 4 {
                    std::thread::sleep(std::time::Duration::from_millis(5 * (attempt + 1) as u64));
                }
            }
        }
    }

    let _ = std::fs::remove_file(&tmp_path);
    Err(last_err
        .unwrap_or_else(|| std::io::Error::other("Failed to rename temporary file atomically")))
}

pub fn save_persisted_session_state(state: &SessionState) {
    let path = get_session_state_path();
    let mut updated = state.clone();
    if updated.updated_at_unix == 0 {
        updated.updated_at_unix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }
    if let Ok(json) = serde_json::to_string_pretty(&updated) {
        let _ = write_atomic(&path, json.as_bytes());
    }
}
