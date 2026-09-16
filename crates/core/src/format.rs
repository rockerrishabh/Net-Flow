use serde::{Deserialize, Serialize};

/// Speed display unit preference.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeedUnit {
    /// Auto-scale based on magnitude (default behavior).
    #[default]
    Auto,
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
}

impl SpeedUnit {
    /// Parse from a string value (used in settings card form data).
    pub fn from_str_value(s: &str) -> Self {
        match s {
            "b" | "bytes" => SpeedUnit::Bytes,
            "kb" | "kilobytes" => SpeedUnit::Kilobytes,
            "mb" | "megabytes" => SpeedUnit::Megabytes,
            "gb" | "gigabytes" => SpeedUnit::Gigabytes,
            _ => SpeedUnit::Auto,
        }
    }

    /// Convert to string value for settings card form data.
    pub fn to_str_value(self) -> &'static str {
        match self {
            SpeedUnit::Auto => "auto",
            SpeedUnit::Bytes => "b",
            SpeedUnit::Kilobytes => "kb",
            SpeedUnit::Megabytes => "mb",
            SpeedUnit::Gigabytes => "gb",
        }
    }
}

/// Format bandwidth in bytes per second into human-readable string.
/// Auto-scales: B/s -> KB/s -> MB/s -> GB/s (using 1024 base).
/// Always formats to 2 decimal places, except B/s which formats to 0 decimal places.
pub fn format_bandwidth(bytes_per_sec: f64) -> String {
    format_bandwidth_with_unit(bytes_per_sec, SpeedUnit::Auto)
}

/// Format bandwidth with a specific unit preference.
pub fn format_bandwidth_with_unit(bytes_per_sec: f64, unit: SpeedUnit) -> String {
    if bytes_per_sec.is_nan() || bytes_per_sec <= 0.0 {
        return match unit {
            SpeedUnit::Bytes => "0 B/s".to_string(),
            SpeedUnit::Kilobytes => "0.00 KB/s".to_string(),
            SpeedUnit::Megabytes => "0.00 MB/s".to_string(),
            SpeedUnit::Gigabytes => "0.00 GB/s".to_string(),
            SpeedUnit::Auto => "0 B/s".to_string(),
        };
    }

    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    match unit {
        SpeedUnit::Bytes => format!("{:.0} B/s", bytes_per_sec),
        SpeedUnit::Kilobytes => format!("{:.2} KB/s", bytes_per_sec / KB),
        SpeedUnit::Megabytes => format!("{:.2} MB/s", bytes_per_sec / MB),
        SpeedUnit::Gigabytes => format!("{:.2} GB/s", bytes_per_sec / GB),
        SpeedUnit::Auto => {
            if bytes_per_sec >= GB {
                format!("{:.2} GB/s", bytes_per_sec / GB)
            } else if bytes_per_sec >= MB {
                format!("{:.2} MB/s", bytes_per_sec / MB)
            } else if bytes_per_sec >= KB {
                format!("{:.2} KB/s", bytes_per_sec / KB)
            } else {
                format!("{:.0} B/s", bytes_per_sec)
            }
        }
    }
}

/// Format total transferred bytes into human-readable string.
pub fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;
    const TB: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;
    if b >= TB {
        format!("{:.2} TB", b / TB)
    } else if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.2} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bandwidth_zero_and_negative() {
        assert_eq!(format_bandwidth(0.0), "0 B/s");
        assert_eq!(format_bandwidth(-10.0), "0 B/s");
        assert_eq!(format_bandwidth(f64::NAN), "0 B/s");
    }

    #[test]
    fn test_format_bandwidth_scales() {
        assert_eq!(format_bandwidth(500.0), "500 B/s");
        assert_eq!(format_bandwidth(1024.0), "1.00 KB/s");
        assert_eq!(format_bandwidth(1536.0), "1.50 KB/s");
        assert_eq!(format_bandwidth(1048576.0), "1.00 MB/s");
        assert_eq!(format_bandwidth(15728640.0), "15.00 MB/s");
        assert_eq!(format_bandwidth(1073741824.0), "1.00 GB/s");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576 * 50), "50.00 MB");
        assert_eq!(format_bytes(1073741824 * 2), "2.00 GB");
    }

    #[test]
    fn test_format_bandwidth_with_unit_fixed() {
        // Force KB display for a value that auto would show as MB
        assert_eq!(
            format_bandwidth_with_unit(1048576.0, SpeedUnit::Kilobytes),
            "1024.00 KB/s"
        );
        // Force MB display for bytes
        assert_eq!(
            format_bandwidth_with_unit(500.0, SpeedUnit::Megabytes),
            "0.00 MB/s"
        );
        // Force Bytes display
        assert_eq!(
            format_bandwidth_with_unit(1048576.0, SpeedUnit::Bytes),
            "1048576 B/s"
        );
    }

    #[test]
    fn test_speed_unit_roundtrip() {
        for unit in [
            SpeedUnit::Auto,
            SpeedUnit::Bytes,
            SpeedUnit::Kilobytes,
            SpeedUnit::Megabytes,
            SpeedUnit::Gigabytes,
        ] {
            assert_eq!(SpeedUnit::from_str_value(unit.to_str_value()), unit);
        }
    }
}
