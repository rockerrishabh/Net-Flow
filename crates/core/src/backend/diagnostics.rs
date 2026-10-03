use super::state::NetworkBackend;
use super::types::LatencyHealth;
use super::wchar_to_string;

pub fn current_utc_iso8601() -> String {
    #[repr(C)]
    struct Win32SystemTime {
        year: u16,
        month: u16,
        day_of_week: u16,
        day: u16,
        hour: u16,
        minute: u16,
        second: u16,
        milliseconds: u16,
    }
    unsafe {
        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetSystemTime(lpSystemTime: *mut Win32SystemTime);
        }
        let mut st = std::mem::zeroed();
        GetSystemTime(&mut st);
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            st.year, st.month, st.day, st.hour, st.minute, st.second
        )
    }
}

pub fn local_timezone_id() -> String {
    #[repr(C)]
    struct DynamicTimeZoneInformation {
        bias: i32,
        standard_name: [u16; 32],
        standard_date: [u16; 8],
        standard_bias: i32,
        daylight_name: [u16; 32],
        daylight_date: [u16; 8],
        daylight_bias: i32,
        time_zone_key_name: [u16; 128],
        dynamic_daylight_time_disabled: u8,
    }
    unsafe {
        #[link(name = "kernel32")]
        unsafe extern "system" {
            fn GetDynamicTimeZoneInformation(
                pTimeZoneInformation: *mut DynamicTimeZoneInformation,
            ) -> u32;
        }
        let mut tz = std::mem::zeroed();
        let res = GetDynamicTimeZoneInformation(&mut tz);
        if res != 0xFFFFFFFF {
            let key = wchar_to_string(&tz.time_zone_key_name);
            let trimmed = key.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
            let std_name = wchar_to_string(&tz.standard_name);
            let trimmed_std = std_name.trim();
            if !trimmed_std.is_empty() {
                return trimmed_std.to_string();
            }
        }
    }
    "UTC".to_string()
}

pub fn format_unix_timestamp_utc(unix_secs: u64) -> String {
    let secs_per_day = 86400u64;
    let days = (unix_secs / secs_per_day) as i64;
    let rem_secs = (unix_secs % secs_per_day) as u32;

    let hour = rem_secs / 3600;
    let minute = (rem_secs % 3600) / 60;
    let second = rem_secs % 60;

    let z = days + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u32;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        y, m, d, hour, minute, second
    )
}

pub fn build_diagnostics_snapshot(
    backend: &NetworkBackend,
    active_adapter: Option<crate::export::AdapterDiagnostics>,
    other_adapters: Vec<crate::export::AdapterSummary>,
) -> crate::export::DiagnosticsSnapshot {
    let snapshot_time = current_utc_iso8601();
    let export_time = snapshot_time.clone();

    let arch = {
        #[cfg(target_arch = "x86_64")]
        {
            crate::export::Architecture::X64
        }
        #[cfg(target_arch = "aarch64")]
        {
            crate::export::Architecture::Arm64
        }
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        {
            crate::export::Architecture::Unknown
        }
    };

    let today_ymd = crate::budget::current_local_ymd();
    let end_date = crate::budget::format_ymd(today_ymd.0, today_ymd.1, today_ymd.2);
    let (history_records, start_date) =
        crate::export::zero_fill_history(&backend.daily_usage.entries, &end_date, 90);

    let history_metadata = crate::export::HistoryMetadata {
        timezone: local_timezone_id(),
        start_date,
        end_date,
        days: 90,
    };

    let privacy = crate::export::PrivacyMetadata::default();

    let now_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let duration_seconds = now_unix.saturating_sub(backend.session_start_unix);
    let session_start_utc = format_unix_timestamp_utc(backend.session_start_unix);
    let bytes_total = backend.session_rx.saturating_add(backend.session_tx);

    let budget_snap = crate::budget::calculate_budget_snapshot(
        &backend.budget_config,
        &mut backend.daily_usage.clone(),
        today_ymd,
    );

    let session = crate::export::SessionDiagnostics {
        session_start_utc,
        duration_seconds,
        bytes_downloaded: backend.session_rx,
        bytes_uploaded: backend.session_tx,
        bytes_total,
        peak_download_bps: backend.peak_rx.round() as u64,
        peak_upload_bps: backend.peak_tx.round() as u64,
        budget_cap_bytes: backend.budget_config.monthly_cap_bytes,
        budget_consumed_bytes: budget_snap.consumed_bytes,
        budget_usage_pct: budget_snap.usage_pct,
        budget_days_remaining: budget_snap.days_remaining,
    };

    let latencies = backend.packet_loss_tracker.latencies_ms();
    let rtt_jitter_ms = if latencies.len() >= 2 {
        crate::export::calculate_mean_absolute_rtt_difference(&latencies)
    } else if let Some(j) = backend.latency.jitter_ms {
        j as f64
    } else {
        0.0
    };

    let (lost_pkts, total_pkts) = backend.packet_loss_tracker.counts();
    let loss_pct = backend.packet_loss_tracker.loss_pct().unwrap_or(0) as f64;
    let rtt_ms = backend.latency.latency_ms.unwrap_or(0) as f64;
    let semantic_health = match backend.packet_loss_tracker.health() {
        LatencyHealth::Healthy => crate::export::SemanticHealth::Healthy,
        LatencyHealth::Degraded => crate::export::SemanticHealth::Degraded,
        LatencyHealth::Timeout => crate::export::SemanticHealth::Timeout,
        LatencyHealth::Unavailable => crate::export::SemanticHealth::Unavailable,
    };

    let quality = crate::export::QualityDiagnostics {
        target_host: backend.latency.target.label().to_string(),
        rtt_ms,
        rtt_jitter_ms,
        jitter_method: "mean_absolute_rtt_difference".to_string(),
        sample_count: total_pkts as usize,
        packets_received: total_pkts.saturating_sub(lost_pkts) as usize,
        packets_lost: lost_pkts as usize,
        packet_loss_percent: loss_pct,
        semantic_health,
    };

    let mut processes = Vec::new();
    for app in &backend.cached_raw_apps.0 {
        processes.push(crate::export::ProcessAttributionRecord {
            process_name: app.process_name.clone(),
            download_bps: app.rx_bps.round() as u64,
            upload_bps: app.tx_bps.round() as u64,
            socket_count: app.connection_count as u32,
        });
    }

    crate::export::DiagnosticsSnapshot {
        schema_version: crate::export::DIAGNOSTICS_SCHEMA_VERSION,
        app_version: env!("CARGO_PKG_VERSION").to_string(),
        architecture: arch,
        snapshot_timestamp_utc: snapshot_time,
        export_generated_at_utc: export_time,
        history_metadata,
        privacy,
        active_adapter,
        other_adapters,
        session,
        quality,
        history: history_records,
        processes,
    }
}
