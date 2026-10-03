use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::types::{
    IpProtocol, Ipv6Route, LatencyHealth, LatencySnapshot, LatencyState, LatencyTarget,
    LatencyTargetMode, ProbeResult,
};

/// Circular sliding-window buffer tracking recent probe results to compute rolling packet loss.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PacketLossTracker {
    pub samples: [Option<ProbeResult>; 20],
    pub head: usize,
    pub count: usize,
}

impl PacketLossTracker {
    pub fn record(&mut self, result: ProbeResult) {
        self.samples[self.head] = Some(result);
        self.head = (self.head + 1) % 20;
        self.count = (self.count + 1).min(20);
    }

    /// Calculates rolling packet loss percentage and packet counts.
    /// Unavailable attempts are excluded from packet loss accounting.
    pub fn evaluate(&self) -> (Option<u8>, u32, u32, LatencyHealth) {
        let mut total_attempts = 0u32;
        let mut lost_packets = 0u32;

        for i in 0..self.count {
            if let Some(res) = self.samples[i] {
                match res {
                    ProbeResult::Success { .. } => {
                        total_attempts += 1;
                    }
                    ProbeResult::Timeout => {
                        total_attempts += 1;
                        lost_packets += 1;
                    }
                    ProbeResult::Unavailable => {}
                }
            }
        }

        if total_attempts == 0 {
            return (None, 0, 0, LatencyHealth::Unavailable);
        }

        let pct = ((lost_packets as f64 / total_attempts as f64) * 100.0).round() as u8;
        let health = if lost_packets == 0 {
            LatencyHealth::Healthy
        } else if lost_packets == total_attempts {
            LatencyHealth::Timeout
        } else {
            LatencyHealth::Degraded
        };

        (Some(pct), lost_packets, total_attempts, health)
    }

    /// Convenience getter for rolling packet loss percentage.
    pub fn loss_pct(&self) -> Option<u8> {
        self.evaluate().0
    }

    /// Convenience getter for (lost_packets, total_attempts).
    pub fn counts(&self) -> (u32, u32) {
        let (_, lost, total, _) = self.evaluate();
        (lost, total)
    }

    /// Convenience getter for latency health assessment.
    pub fn health(&self) -> LatencyHealth {
        self.evaluate().3
    }

    /// Extracts successful probe latencies in chronological order in milliseconds.
    pub fn latencies_ms(&self) -> Vec<f64> {
        let mut result = Vec::with_capacity(self.count);
        let start = (self.head + 20 - self.count) % 20;
        for i in 0..self.count {
            let idx = (start + i) % 20;
            if let Some(ProbeResult::Success { latency }) = self.samples[idx] {
                result.push(latency.as_secs_f64() * 1000.0);
            }
        }
        result
    }
}

/// Query the active default IPv4 gateway address.
///
/// Uses Windows `GetBestRoute2` towards public internet (`1.1.1.1`) to determine
/// the exact next-hop gateway Windows uses on the active network route.
/// Falls back to scanning default route entries (`0.0.0.0/0`) via `GetIpForwardTable2`.
pub fn query_ipv4_gateway_address(interface_luid: Option<u64>) -> Option<std::net::Ipv4Addr> {
    use windows::Win32::NetworkManagement::IpHelper::{
        FreeMibTable, GetBestRoute2, GetIpForwardTable2, MIB_IPFORWARD_ROW2, MIB_IPFORWARD_TABLE2,
    };
    use windows::Win32::NetworkManagement::Ndis::NET_LUID_LH;
    use windows::Win32::Networking::WinSock::{AF_INET, SOCKADDR_INET};

    unsafe {
        // Destination address: 1.1.1.1 (public internet probe route)
        let mut dest: SOCKADDR_INET = std::mem::zeroed();
        dest.si_family = AF_INET;
        dest.Ipv4.sin_family = AF_INET;
        dest.Ipv4.sin_addr.S_un.S_addr = u32::from_ne_bytes([1, 1, 1, 1]);

        let mut best_route: MIB_IPFORWARD_ROW2 = std::mem::zeroed();
        let mut best_source: SOCKADDR_INET = std::mem::zeroed();

        let luid_val = interface_luid.map(|l| NET_LUID_LH { Value: l });
        let luid_ptr = luid_val.as_ref().map(|l| l as *const _);

        let status = GetBestRoute2(
            luid_ptr,
            0,
            None,
            &dest,
            0,
            &mut best_route,
            &mut best_source,
        );

        if status.0 == 0 && best_route.NextHop.si_family == AF_INET {
            let s_addr = best_route.NextHop.Ipv4.sin_addr.S_un.S_addr;
            if s_addr != 0 {
                return Some(std::net::Ipv4Addr::from(s_addr.to_ne_bytes()));
            }
        }

        // Fallback: enumerate IPv4 forwarding table for default 0.0.0.0/0 route
        let mut table: *mut MIB_IPFORWARD_TABLE2 = std::ptr::null_mut();
        if GetIpForwardTable2(AF_INET, &mut table).0 == 0 && !table.is_null() {
            let entries = (*table).NumEntries as usize;
            let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), entries);
            let mut best_gateway = None;
            let mut lowest_metric = u32::MAX;

            for row in rows {
                if let Some(target_luid) = interface_luid
                    && row.InterfaceLuid.Value != target_luid
                {
                    continue;
                }
                if row.DestinationPrefix.PrefixLength == 0 && row.NextHop.si_family == AF_INET {
                    let gw = row.NextHop.Ipv4.sin_addr.S_un.S_addr;
                    if gw != 0 && row.Metric < lowest_metric {
                        lowest_metric = row.Metric;
                        best_gateway = Some(std::net::Ipv4Addr::from(gw.to_ne_bytes()));
                    }
                }
            }

            FreeMibTable(table as *const core::ffi::c_void);
            if best_gateway.is_some() {
                return best_gateway;
            }
        }
    }

    None
}

/// Query the active default IPv6 route and gateway using `GetBestRoute2`.
///
/// Uses Windows `GetBestRoute2` towards Cloudflare IPv6 DNS (`2606:4700:4700::1111`) to determine
/// the preferred interface, best source address, and next-hop IPv6 gateway.
/// Falls back to scanning default route entries (`::/0`) via `GetIpForwardTable2`.
pub fn query_ipv6_route(interface_luid: Option<u64>) -> Option<Ipv6Route> {
    use windows::Win32::NetworkManagement::IpHelper::{
        FreeMibTable, GetBestRoute2, GetIpForwardTable2, MIB_IPFORWARD_ROW2, MIB_IPFORWARD_TABLE2,
    };
    use windows::Win32::NetworkManagement::Ndis::NET_LUID_LH;
    use windows::Win32::Networking::WinSock::{AF_INET6, IN6_ADDR, SOCKADDR_INET};

    unsafe {
        let mut dest: SOCKADDR_INET = std::mem::zeroed();
        dest.si_family = AF_INET6;
        dest.Ipv6.sin6_family = AF_INET6;
        dest.Ipv6.sin6_addr = IN6_ADDR {
            u: windows::Win32::Networking::WinSock::IN6_ADDR_0 {
                Byte: [
                    0x26, 0x06, 0x47, 0x00, 0x47, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x11, 0x11,
                ],
            },
        };

        let mut best_route: MIB_IPFORWARD_ROW2 = std::mem::zeroed();
        let mut best_source: SOCKADDR_INET = std::mem::zeroed();

        let luid_val = interface_luid.map(|l| NET_LUID_LH { Value: l });
        let luid_ptr = luid_val.as_ref().map(|l| l as *const _);

        let status = GetBestRoute2(
            luid_ptr,
            0,
            None,
            &dest,
            0,
            &mut best_route,
            &mut best_source,
        );

        if status.0 == 0 {
            let source_ip = std::net::Ipv6Addr::from(best_source.Ipv6.sin6_addr.u.Byte);
            let gateway_ip = if best_route.NextHop.si_family == AF_INET6 {
                let bytes = best_route.NextHop.Ipv6.sin6_addr.u.Byte;
                let addr = std::net::Ipv6Addr::from(bytes);
                if !addr.is_unspecified() {
                    Some(addr)
                } else {
                    None
                }
            } else {
                None
            };

            return Some(Ipv6Route {
                interface_luid: best_route.InterfaceLuid.Value,
                interface_index: best_route.InterfaceIndex,
                source: source_ip,
                gateway: gateway_ip,
            });
        }

        // Fallback: check IPv6 forward table for default ::/0 route
        let mut table: *mut MIB_IPFORWARD_TABLE2 = std::ptr::null_mut();
        if GetIpForwardTable2(AF_INET6, &mut table).0 == 0 && !table.is_null() {
            let entries = (*table).NumEntries as usize;
            let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), entries);
            let mut best_route_match = None;
            let mut lowest_metric = u32::MAX;

            for row in rows {
                if let Some(target_luid) = interface_luid
                    && row.InterfaceLuid.Value != target_luid
                {
                    continue;
                }
                if row.DestinationPrefix.PrefixLength == 0 && row.NextHop.si_family == AF_INET6 {
                    let bytes = row.NextHop.Ipv6.sin6_addr.u.Byte;
                    let addr = std::net::Ipv6Addr::from(bytes);
                    if !addr.is_unspecified() && row.Metric < lowest_metric {
                        lowest_metric = row.Metric;
                        best_route_match = Some(Ipv6Route {
                            interface_luid: row.InterfaceLuid.Value,
                            interface_index: row.InterfaceIndex,
                            source: std::net::Ipv6Addr::UNSPECIFIED,
                            gateway: Some(addr),
                        });
                    }
                }
            }

            FreeMibTable(table as *const core::ffi::c_void);
            if best_route_match.is_some() {
                return best_route_match;
            }
        }
    }

    None
}

/// Probes round-trip latency to the given IPv4 target using asynchronous Win32 `IcmpSendEcho2`.
pub fn probe_latency_ipv4(target_ip: std::net::Ipv4Addr, timeout_ms: u32) -> ProbeResult {
    use windows::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0, WAIT_TIMEOUT};
    use windows::Win32::NetworkManagement::IpHelper::{
        ICMP_ECHO_REPLY, IcmpCloseHandle, IcmpCreateFile, IcmpParseReplies, IcmpSendEcho2,
    };
    use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};

    unsafe {
        let icmp_handle = match IcmpCreateFile() {
            Ok(h) if !h.is_invalid() => h,
            _ => return ProbeResult::Unavailable,
        };

        let event = match CreateEventW(None, false, false, None) {
            Ok(h) => h,
            Err(_) => {
                let _ = IcmpCloseHandle(icmp_handle);
                return ProbeResult::Unavailable;
            }
        };

        let reply_buf_len = size_of::<ICMP_ECHO_REPLY>() + 64;
        let mut reply_buf = vec![0u8; reply_buf_len];
        let send_data = *b"NetFlow";
        let dest_addr = u32::from_ne_bytes(target_ip.octets());

        let _ = IcmpSendEcho2(
            icmp_handle,
            Some(event),
            None,
            None,
            dest_addr,
            send_data.as_ptr() as *const _,
            send_data.len() as u16,
            None,
            reply_buf.as_mut_ptr() as *mut _,
            reply_buf_len as u32,
            timeout_ms,
        );

        let wait_result = WaitForSingleObject(event, timeout_ms);
        let _ = CloseHandle(event);

        if wait_result == WAIT_OBJECT_0 {
            let parse_count =
                IcmpParseReplies(reply_buf.as_mut_ptr() as *mut _, reply_buf_len as u32);
            let _ = IcmpCloseHandle(icmp_handle);

            if parse_count > 0 {
                let reply = &*(reply_buf.as_ptr() as *const ICMP_ECHO_REPLY);
                if reply.Status == 0 {
                    ProbeResult::Success {
                        latency: Duration::from_millis(reply.RoundTripTime as u64),
                    }
                } else {
                    ProbeResult::Timeout
                }
            } else {
                ProbeResult::Timeout
            }
        } else if wait_result == WAIT_TIMEOUT {
            let _ = IcmpCloseHandle(icmp_handle);
            ProbeResult::Timeout
        } else {
            let _ = IcmpCloseHandle(icmp_handle);
            ProbeResult::Unavailable
        }
    }
}

/// Probes round-trip latency to the given IPv6 target using asynchronous Win32 `Icmp6SendEcho2`.
pub fn probe_latency_ipv6(target_ip: std::net::Ipv6Addr, timeout_ms: u32) -> ProbeResult {
    use windows::Win32::Foundation::{CloseHandle, WAIT_OBJECT_0, WAIT_TIMEOUT};
    use windows::Win32::NetworkManagement::IpHelper::{
        ICMPV6_ECHO_REPLY_LH, Icmp6CreateFile, Icmp6ParseReplies, Icmp6SendEcho2, IcmpCloseHandle,
    };
    use windows::Win32::Networking::WinSock::{AF_INET6, IN6_ADDR, SOCKADDR_IN6};
    use windows::Win32::System::Threading::{CreateEventW, WaitForSingleObject};

    unsafe {
        let icmp_handle = match Icmp6CreateFile() {
            Ok(h) if !h.is_invalid() => h,
            _ => return ProbeResult::Unavailable,
        };

        let event = match CreateEventW(None, false, false, None) {
            Ok(h) => h,
            Err(_) => {
                let _ = IcmpCloseHandle(icmp_handle);
                return ProbeResult::Unavailable;
            }
        };

        let reply_buf_len = size_of::<ICMPV6_ECHO_REPLY_LH>() + 128;
        let mut reply_buf = vec![0u8; reply_buf_len];
        let send_data = *b"NetFlow";

        let mut dest: SOCKADDR_IN6 = std::mem::zeroed();
        dest.sin6_family = AF_INET6;
        dest.sin6_addr = IN6_ADDR {
            u: windows::Win32::Networking::WinSock::IN6_ADDR_0 {
                Byte: target_ip.octets(),
            },
        };

        let mut source: SOCKADDR_IN6 = std::mem::zeroed();
        source.sin6_family = AF_INET6;

        let _ = Icmp6SendEcho2(
            icmp_handle,
            Some(event),
            None,
            None,
            &source,
            &dest,
            send_data.as_ptr() as *const _,
            send_data.len() as u16,
            None,
            reply_buf.as_mut_ptr() as *mut _,
            reply_buf_len as u32,
            timeout_ms,
        );

        let wait_result = WaitForSingleObject(event, timeout_ms);
        let _ = CloseHandle(event);

        if wait_result == WAIT_OBJECT_0 {
            let parse_count =
                Icmp6ParseReplies(reply_buf.as_mut_ptr() as *mut _, reply_buf_len as u32);
            let _ = IcmpCloseHandle(icmp_handle);

            if parse_count > 0 {
                let reply = &*(reply_buf.as_ptr() as *const ICMPV6_ECHO_REPLY_LH);
                if reply.Status == 0 {
                    ProbeResult::Success {
                        latency: Duration::from_millis(reply.RoundTripTime as u64),
                    }
                } else {
                    ProbeResult::Timeout
                }
            } else {
                ProbeResult::Timeout
            }
        } else if wait_result == WAIT_TIMEOUT {
            let _ = IcmpCloseHandle(icmp_handle);
            ProbeResult::Timeout
        } else {
            let _ = IcmpCloseHandle(icmp_handle);
            ProbeResult::Unavailable
        }
    }
}

/// Platform-neutral internal probe function dispatching between IPv4 and IPv6 ICMP echo APIs.
pub fn probe_latency(target: std::net::IpAddr, timeout_ms: u32) -> ProbeResult {
    match target {
        std::net::IpAddr::V4(ipv4) => probe_latency_ipv4(ipv4, timeout_ms),
        std::net::IpAddr::V6(ipv6) => probe_latency_ipv6(ipv6, timeout_ms),
    }
}

/// Probes round-trip latency using dual-stack IPv6-first routing with seamless IPv4 fallback.
/// Returns the constructed `LatencySnapshot` along with the individual `ProbeResult`.
pub fn sample_latency_snapshot_dual_stack(
    mode: LatencyTargetMode,
    sequence: u64,
    timeout_ms: u32,
    prev_ms: Option<u32>,
) -> (LatencySnapshot, ProbeResult) {
    let now_unix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let ipv6_route = query_ipv6_route(None);
    let ipv4_gateway = query_ipv4_gateway_address(None);

    let (target, primary_ipv6, fallback_ipv4) = match mode {
        LatencyTargetMode::Internet => (
            LatencyTarget::Internet,
            Some(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111,
            ))),
            Some(std::net::IpAddr::V4(std::net::Ipv4Addr::new(1, 1, 1, 1))),
        ),
        LatencyTargetMode::Gateway => {
            let gw_v6 = ipv6_route
                .as_ref()
                .and_then(|r| r.gateway)
                .map(std::net::IpAddr::V6);
            let gw_v4 = ipv4_gateway.map(std::net::IpAddr::V4);
            (LatencyTarget::Gateway, gw_v6, gw_v4)
        }
        LatencyTargetMode::Auto => {
            if let Some(gw_v6) = ipv6_route.as_ref().and_then(|r| r.gateway) {
                (
                    LatencyTarget::Gateway,
                    Some(std::net::IpAddr::V6(gw_v6)),
                    ipv4_gateway.map(std::net::IpAddr::V4),
                )
            } else if let Some(gw_v4) = ipv4_gateway {
                (
                    LatencyTarget::Gateway,
                    None,
                    Some(std::net::IpAddr::V4(gw_v4)),
                )
            } else {
                (
                    LatencyTarget::Internet,
                    Some(std::net::IpAddr::V6(std::net::Ipv6Addr::new(
                        0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111,
                    ))),
                    Some(std::net::IpAddr::V4(std::net::Ipv4Addr::new(1, 1, 1, 1))),
                )
            }
        }
    };

    // 1. Try IPv6 first if route / target is available
    if let Some(ip6) = primary_ipv6 {
        let res = probe_latency(ip6, timeout_ms);
        match res {
            ProbeResult::Success { latency } => {
                let ms = latency.as_millis() as u32;
                let jitter = prev_ms.map(|p| ms.abs_diff(p));
                let snap = LatencySnapshot {
                    latency_ms: Some(ms),
                    state: LatencyState::Healthy,
                    target,
                    sequence,
                    sampled_at_unix: now_unix,
                    jitter_ms: jitter,
                    protocol: Some(IpProtocol::Ipv6),
                    packet_loss_pct: None,
                    pings_lost: 0,
                    pings_total: 0,
                    health: LatencyHealth::Healthy,
                };
                return (snap, res);
            }
            ProbeResult::Unavailable => {
                // IPv6 route unavailable -> fall back to IPv4
            }
            ProbeResult::Timeout => {
                // If IPv6 timed out, test if IPv4 has connectivity
                if let Some(ip4) = fallback_ipv4 {
                    let res4 = probe_latency(ip4, timeout_ms);
                    if let ProbeResult::Success { latency } = res4 {
                        let ms = latency.as_millis() as u32;
                        let jitter = prev_ms.map(|p| ms.abs_diff(p));
                        let snap = LatencySnapshot {
                            latency_ms: Some(ms),
                            state: LatencyState::Healthy,
                            target,
                            sequence,
                            sampled_at_unix: now_unix,
                            jitter_ms: jitter,
                            protocol: Some(IpProtocol::Ipv4),
                            packet_loss_pct: None,
                            pings_lost: 0,
                            pings_total: 0,
                            health: LatencyHealth::Healthy,
                        };
                        return (snap, res4);
                    }
                }
                let snap = LatencySnapshot {
                    latency_ms: None,
                    state: LatencyState::Timeout,
                    target,
                    sequence,
                    sampled_at_unix: now_unix,
                    jitter_ms: None,
                    protocol: Some(IpProtocol::Ipv6),
                    packet_loss_pct: None,
                    pings_lost: 0,
                    pings_total: 0,
                    health: LatencyHealth::Timeout,
                };
                return (snap, ProbeResult::Timeout);
            }
        }
    }

    // 2. IPv4 probe fallback
    if let Some(ip4) = fallback_ipv4 {
        let res = probe_latency(ip4, timeout_ms);
        let (state, ms) = match res {
            ProbeResult::Success { latency } => {
                (LatencyState::Healthy, Some(latency.as_millis() as u32))
            }
            ProbeResult::Timeout => (LatencyState::Timeout, None),
            ProbeResult::Unavailable => (LatencyState::Unavailable, None),
        };
        let jitter = ms.and_then(|m| prev_ms.map(|p| m.abs_diff(p)));
        let health = match state {
            LatencyState::Healthy => LatencyHealth::Healthy,
            LatencyState::Timeout => LatencyHealth::Timeout,
            _ => LatencyHealth::Unavailable,
        };
        let snap = LatencySnapshot {
            latency_ms: ms,
            state,
            target,
            sequence,
            sampled_at_unix: now_unix,
            jitter_ms: jitter,
            protocol: Some(IpProtocol::Ipv4),
            packet_loss_pct: None,
            pings_lost: 0,
            pings_total: 0,
            health,
        };
        (snap, res)
    } else {
        let snap = LatencySnapshot {
            latency_ms: None,
            state: LatencyState::Unavailable,
            target,
            sequence,
            sampled_at_unix: now_unix,
            jitter_ms: None,
            protocol: None,
            packet_loss_pct: None,
            pings_lost: 0,
            pings_total: 0,
            health: LatencyHealth::Unavailable,
        };
        (snap, ProbeResult::Unavailable)
    }
}

/// Convenience method that resolves the active probe target according to `LatencyTargetMode`
/// and queries round-trip latency, returning a complete `LatencySnapshot`.
pub fn sample_latency_snapshot(
    mode: LatencyTargetMode,
    sequence: u64,
    timeout_ms: u32,
) -> LatencySnapshot {
    sample_latency_snapshot_dual_stack(mode, sequence, timeout_ms, None).0
}
