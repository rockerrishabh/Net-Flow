use super::phy::{
    WLAN_CONNECTION_ATTRIBUTES, WLAN_INTERFACE_INFO_LIST, WLAN_INTF_OPCODE_CHANNEL_NUMBER,
    WLAN_INTF_OPCODE_CURRENT_CONNECTION, WLAN_INTF_OPCODE_REALTIME_CONNECTION_QUALITY,
    WLAN_REALTIME_CONNECTION_QUALITY, WlanCloseHandle, WlanEnumInterfaces, WlanFreeMemory,
    WlanOpenHandle, WlanQueryInterface, luid_for_wifi_guid,
};
use super::types::{
    InterfaceCategory, InterfaceInfo, InterfaceMedium, signal_quality_to_rssi_dbm, wifi_band,
    wifi_generation_with_band,
};
use super::wchar_to_string;

/// Identifies adapter category and hardware medium from Windows NDIS driver properties and name strings.
pub fn classify_interface(
    if_type: i32,
    tunnel_type: i32,
    description: &str,
    alias: &str,
) -> (InterfaceCategory, InterfaceMedium) {
    const IF_TYPE_SOFTWARE_LOOPBACK: i32 = 24;
    const IF_TYPE_ETHERNET_CSMACD: i32 = 6;
    const IF_TYPE_IEEE80211: i32 = 71;
    const IF_TYPE_GIGABITETHERNET: i32 = 117;
    const IF_TYPE_FASTETHER: i32 = 62;
    const IF_TYPE_TUNNEL: i32 = 131;
    const IF_TYPE_WWANPP: i32 = 243;
    const IF_TYPE_WWANPP2: i32 = 244;

    if if_type == IF_TYPE_SOFTWARE_LOOPBACK {
        return (InterfaceCategory::Loopback, InterfaceMedium::Loopback);
    }

    if tunnel_type != 0 || if_type == IF_TYPE_TUNNEL {
        return (InterfaceCategory::Tunnel, InterfaceMedium::Virtual);
    }

    let combined = format!("{} {}", description, alias).to_lowercase();
    if combined.contains("virtual")
        || combined.contains("hyper-v")
        || combined.contains("vethernet")
        || combined.contains("vpn")
        || combined.contains("tap-")
        || combined.contains("wsl")
        || combined.contains("docker")
        || combined.contains("vmware")
        || combined.contains("virtualbox")
        || combined.contains("npcap")
        || combined.contains("wan miniport")
        || combined.contains("pacer")
        || combined.contains("loopback")
        || combined.contains("bluetooth")
        || combined.contains("filter")
        || combined.contains("wfp")
        || combined.contains("lightweight")
        || combined.contains("packet scheduler")
        || combined.contains("qos")
        || combined.contains("ndiscap")
        || combined.contains("native mac layer")
    {
        return (InterfaceCategory::Virtual, InterfaceMedium::Virtual);
    }

    if if_type == IF_TYPE_IEEE80211
        || combined.contains("wi-fi")
        || combined.contains("wifi")
        || combined.contains("wireless")
        || combined.contains("802.11")
        || combined.contains("wlan")
    {
        return (InterfaceCategory::Physical, InterfaceMedium::Wifi);
    }

    if if_type == IF_TYPE_WWANPP
        || if_type == IF_TYPE_WWANPP2
        || combined.contains("cellular")
        || combined.contains("mobile broadband")
        || combined.contains("wwan")
        || combined.contains(" lte")
        || combined.contains("lte ")
        || combined.contains(" 5g")
        || combined.contains("5g ")
    {
        return (InterfaceCategory::Physical, InterfaceMedium::Cellular);
    }

    if if_type == IF_TYPE_ETHERNET_CSMACD
        || if_type == IF_TYPE_GIGABITETHERNET
        || if_type == IF_TYPE_FASTETHER
        || combined.contains("ethernet")
        || combined.contains("gigabit")
        || combined.contains("realtek")
        || combined.contains("intel(r)")
    {
        return (InterfaceCategory::Physical, InterfaceMedium::Ethernet);
    }

    (InterfaceCategory::Physical, InterfaceMedium::Other)
}

/// Polls all network adapters and current octet transfer counters via Windows `GetIfTable2`.
pub fn query_interfaces() -> Result<Vec<InterfaceInfo>, String> {
    use windows::Win32::NetworkManagement::IpHelper;

    let mut table: *mut IpHelper::MIB_IF_TABLE2 = std::ptr::null_mut();
    let status = unsafe { IpHelper::GetIfTable2(&mut table) };

    if status.is_err() || table.is_null() {
        return Err(format!("GetIfTable2 failed: {:?}", status));
    }

    let mut interfaces = Vec::new();
    unsafe {
        let entries = (*table).NumEntries as usize;
        let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), entries);

        for row in rows {
            let luid = row.InterfaceLuid.Value;
            let index = row.InterfaceIndex;
            let desc = wchar_to_string(&row.Description);
            let alias = wchar_to_string(&row.Alias);
            let if_type = row.Type as i32;
            let tunnel_type = row.TunnelType.0;
            let (category, medium) = classify_interface(if_type, tunnel_type, &desc, &alias);
            let oper_status = row.OperStatus.0;

            let clean_name = if !alias.is_empty()
                && !alias.to_lowercase().contains("filter")
                && !alias.to_lowercase().contains("wfp")
            {
                alias
            } else if medium == InterfaceMedium::Wifi {
                "Wi-Fi".to_string()
            } else if medium == InterfaceMedium::Ethernet {
                "Ethernet".to_string()
            } else if medium == InterfaceMedium::Cellular {
                "Cellular".to_string()
            } else if !desc.is_empty() {
                desc.clone()
            } else {
                "Network".to_string()
            };

            interfaces.push(InterfaceInfo {
                luid,
                index,
                name: clean_name,
                description: desc,
                category,
                medium,
                oper_status,
                in_octets: row.InOctets,
                out_octets: row.OutOctets,
                speed: row.ReceiveLinkSpeed,
            });
        }

        IpHelper::FreeMibTable(table as *const core::ffi::c_void);
    }

    Ok(interfaces)
}

#[derive(Debug, Clone)]
pub struct RawAdapterInfo {
    pub name: String,
    pub description: String,
    pub friendly_name: String,
    pub interface_type: crate::export::InterfaceType,
    pub is_physical: bool,
    pub is_up: bool,
    pub routing_metric: u32,
    pub link_speed_bps: u64,
    pub mac_address: String,
    pub ipv4_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
    pub default_gateways: Vec<String>,
    pub dns_servers: Vec<String>,
    pub dhcp_enabled: bool,
    pub dhcp_server: Option<String>,
    pub mtu: u32,
    pub wifi: Option<crate::export::WifiDiagnostics>,
}

pub fn is_physical_adapter(
    interface_type: crate::export::InterfaceType,
    desc: &str,
    friendly: &str,
) -> bool {
    if interface_type == crate::export::InterfaceType::Loopback
        || interface_type == crate::export::InterfaceType::Vpn
    {
        return false;
    }
    let lower_desc = desc.to_lowercase();
    let lower_friendly = friendly.to_lowercase();
    let virtual_keywords = [
        "hyper-v",
        "virtual",
        "wsl",
        "docker",
        "vmware",
        "virtualbox",
        "tap-",
        "vethernet",
        "tailscale",
        "zerotier",
        "wireguard",
        "vpn",
        "loopback",
        "bluetooth",
        "pseudo",
    ];
    for kw in &virtual_keywords {
        if lower_desc.contains(kw) || lower_friendly.contains(kw) {
            return false;
        }
    }
    interface_type == crate::export::InterfaceType::Ethernet
        || interface_type == crate::export::InterfaceType::Wifi
        || interface_type == crate::export::InterfaceType::Cellular
}

pub fn has_usable_ip(ipv4: &[String], ipv6: &[String]) -> bool {
    let has_v4 = ipv4.iter().any(|s| {
        let ip_str = s.split('/').next().unwrap_or(s);
        if let Ok(ip) = ip_str.parse::<std::net::Ipv4Addr>() {
            !ip.is_loopback() && !ip.is_unspecified() && !ip.is_link_local()
        } else {
            false
        }
    });
    if has_v4 {
        return true;
    }
    ipv6.iter().any(|s| {
        let ip_str = s.split('/').next().unwrap_or(s);
        if let Ok(ip) = ip_str.parse::<std::net::Ipv6Addr>() {
            !ip.is_loopback() && !ip.is_unspecified() && !ip.is_unicast_link_local()
        } else {
            false
        }
    })
}

pub fn has_default_gateway(gateways: &[String]) -> bool {
    gateways.iter().any(|s| {
        if let Ok(ip) = s.parse::<std::net::IpAddr>() {
            !ip.is_loopback() && !ip.is_unspecified()
        } else {
            false
        }
    })
}

pub fn select_active_adapter(
    mut adapters: Vec<RawAdapterInfo>,
) -> (
    Option<crate::export::AdapterDiagnostics>,
    Vec<crate::export::AdapterSummary>,
) {
    let mut best_index = None;
    let mut best_score: Option<(bool, bool, bool, bool, u32, u64)> = None;

    for (i, a) in adapters.iter().enumerate() {
        let is_up = a.is_up;
        let has_gateway = has_default_gateway(&a.default_gateways);
        let has_ip = has_usable_ip(&a.ipv4_addresses, &a.ipv6_addresses);
        let is_phys = a.is_physical;
        let metric = a.routing_metric;
        let speed = a.link_speed_bps;

        // Active adapter MUST be operational and possess a usable non-APIPA IP
        if !is_up || !has_ip {
            continue;
        }

        let score = (is_up, has_gateway, has_ip, is_phys, metric, speed);

        match &best_score {
            None => {
                best_score = Some(score);
                best_index = Some(i);
            }
            Some(curr_best) => {
                // Active Adapter Selection Hierarchy:
                // 1. Gateway presence (prefer default route)
                // 2. Physical adapter preference (Ethernet/Wi-Fi over Virtual/Hyper-V)
                // 3. Lowest routing metric
                // 4. Highest link speed
                let is_better = if score.1 != curr_best.1 {
                    score.1 && !curr_best.1
                } else if score.3 != curr_best.3 {
                    score.3 && !curr_best.3
                } else if score.4 != curr_best.4 {
                    score.4 < curr_best.4
                } else {
                    score.5 > curr_best.5
                };

                if is_better {
                    best_score = Some(score);
                    best_index = Some(i);
                }
            }
        }
    }

    let mut active = None;
    let mut summaries = Vec::with_capacity(adapters.len());

    if let Some(idx) = best_index {
        let chosen = adapters.remove(idx);
        active = Some(crate::export::AdapterDiagnostics {
            name: chosen.name,
            description: chosen.description,
            friendly_name: chosen.friendly_name,
            interface_type: chosen.interface_type,
            mac_address: chosen.mac_address,
            ipv4_addresses: chosen.ipv4_addresses,
            ipv6_addresses: chosen.ipv6_addresses,
            default_gateways: chosen.default_gateways,
            dns_servers: chosen.dns_servers,
            dhcp_enabled: chosen.dhcp_enabled,
            dhcp_server: chosen.dhcp_server,
            mtu: chosen.mtu,
            link_speed_bps: chosen.link_speed_bps,
            wifi: chosen.wifi,
        });
    }

    for a in adapters {
        summaries.push(crate::export::AdapterSummary {
            name: a.name,
            friendly_name: a.friendly_name,
            interface_type: a.interface_type,
            is_up: a.is_up,
            ipv4_addresses: a.ipv4_addresses,
        });
    }

    (active, summaries)
}

unsafe fn sockaddr_to_ip(
    sa_ptr: *const windows::Win32::Networking::WinSock::SOCKADDR,
) -> Option<std::net::IpAddr> {
    if sa_ptr.is_null() {
        return None;
    }
    use windows::Win32::Networking::WinSock::{AF_INET, AF_INET6, SOCKADDR_IN, SOCKADDR_IN6};
    unsafe {
        let family = (*sa_ptr).sa_family;
        if family == AF_INET {
            let sin = &*(sa_ptr as *const SOCKADDR_IN);
            let s_addr = sin.sin_addr.S_un.S_addr;
            Some(std::net::IpAddr::V4(std::net::Ipv4Addr::from(
                s_addr.to_ne_bytes(),
            )))
        } else if family == AF_INET6 {
            let sin6 = &*(sa_ptr as *const SOCKADDR_IN6);
            let bytes = sin6.sin6_addr.u.Byte;
            Some(std::net::IpAddr::V6(std::net::Ipv6Addr::from(bytes)))
        } else {
            None
        }
    }
}

pub fn query_raw_adapters() -> Vec<RawAdapterInfo> {
    use windows::Win32::Foundation::ERROR_BUFFER_OVERFLOW;
    use windows::Win32::NetworkManagement::IpHelper::{
        GAA_FLAG_INCLUDE_GATEWAYS, GAA_FLAG_INCLUDE_PREFIX, GetAdaptersAddresses,
        IP_ADAPTER_ADDRESSES_LH,
    };
    use windows::Win32::Networking::WinSock::AF_UNSPEC;

    let mut buf_len = 16384u32;
    let mut buf = vec![0u8; buf_len as usize];
    let flags = GAA_FLAG_INCLUDE_GATEWAYS | GAA_FLAG_INCLUDE_PREFIX;

    let mut ret = unsafe {
        GetAdaptersAddresses(
            AF_UNSPEC.0 as u32,
            flags,
            None,
            Some(buf.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
            &mut buf_len,
        )
    };

    if ret == ERROR_BUFFER_OVERFLOW.0 {
        buf.resize(buf_len as usize, 0);
        ret = unsafe {
            GetAdaptersAddresses(
                AF_UNSPEC.0 as u32,
                flags,
                None,
                Some(buf.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
                &mut buf_len,
            )
        };
    }

    if ret != 0 {
        return Vec::new();
    }

    let mut raw_adapters = Vec::new();
    let mut curr_ptr = buf.as_ptr() as *const IP_ADAPTER_ADDRESSES_LH;

    while !curr_ptr.is_null() {
        let curr = unsafe { &*curr_ptr };

        let (name, friendly_name, description) = unsafe {
            (
                curr.AdapterName.to_string().unwrap_or_default(),
                curr.FriendlyName.to_string().unwrap_or_default(),
                curr.Description.to_string().unwrap_or_default(),
            )
        };
        let if_type = curr.IfType as i32;
        let is_up = curr.OperStatus.0 == 1; // IfOperStatusUp = 1

        let (category, medium) = classify_interface(if_type, 0, &description, &friendly_name);
        let interface_type = match medium {
            InterfaceMedium::Ethernet => crate::export::InterfaceType::Ethernet,
            InterfaceMedium::Wifi => crate::export::InterfaceType::Wifi,
            InterfaceMedium::Cellular => crate::export::InterfaceType::Cellular,
            InterfaceMedium::Loopback => crate::export::InterfaceType::Loopback,
            InterfaceMedium::Virtual => {
                let desc_low = description.to_lowercase();
                let f_low = friendly_name.to_lowercase();
                if desc_low.contains("vpn") || f_low.contains("vpn") {
                    crate::export::InterfaceType::Vpn
                } else {
                    crate::export::InterfaceType::Other
                }
            }
            InterfaceMedium::Other => {
                let desc_low = description.to_lowercase();
                let f_low = friendly_name.to_lowercase();
                if desc_low.contains("vpn") || f_low.contains("vpn") {
                    crate::export::InterfaceType::Vpn
                } else {
                    crate::export::InterfaceType::Other
                }
            }
        };

        let is_physical = category == InterfaceCategory::Physical
            && is_physical_adapter(interface_type, &description, &friendly_name);

        let phys_len = curr.PhysicalAddressLength as usize;
        let mac_address = if phys_len == 6 {
            let p = &curr.PhysicalAddress;
            format!(
                "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                p[0], p[1], p[2], p[3], p[4], p[5]
            )
        } else if phys_len > 0 && phys_len <= 8 {
            curr.PhysicalAddress[..phys_len]
                .iter()
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(":")
        } else {
            String::new()
        };

        let mut ipv4_addresses = Vec::new();
        let mut ipv6_addresses = Vec::new();
        let mut unicast_ptr = curr.FirstUnicastAddress;
        while !unicast_ptr.is_null() {
            let u = unsafe { &*unicast_ptr };
            let prefix = u.OnLinkPrefixLength;
            if let Some(ip) = unsafe { sockaddr_to_ip(u.Address.lpSockaddr) } {
                match ip {
                    std::net::IpAddr::V4(v4) => ipv4_addresses.push(format!("{}/{}", v4, prefix)),
                    std::net::IpAddr::V6(v6) => ipv6_addresses.push(format!("{}/{}", v6, prefix)),
                }
            }
            unicast_ptr = u.Next;
        }

        let mut default_gateways = Vec::new();
        let mut gw_ptr = curr.FirstGatewayAddress;
        while !gw_ptr.is_null() {
            let gw = unsafe { &*gw_ptr };
            if let Some(ip) = unsafe { sockaddr_to_ip(gw.Address.lpSockaddr) } {
                default_gateways.push(ip.to_string());
            }
            gw_ptr = gw.Next;
        }

        let mut dns_servers = Vec::new();
        let mut dns_ptr = curr.FirstDnsServerAddress;
        while !dns_ptr.is_null() {
            let dns = unsafe { &*dns_ptr };
            if let Some(ip) = unsafe { sockaddr_to_ip(dns.Address.lpSockaddr) } {
                dns_servers.push(ip.to_string());
            }
            dns_ptr = dns.Next;
        }

        let dhcp_enabled = (unsafe { curr.Anonymous2.Flags } & 0x0004) != 0;
        let dhcp_server = if dhcp_enabled {
            unsafe { sockaddr_to_ip(curr.Dhcpv4Server.lpSockaddr).map(|ip| ip.to_string()) }
        } else {
            None
        };

        let mtu = curr.Mtu;
        let link_speed_bps = curr.ReceiveLinkSpeed.max(curr.TransmitLinkSpeed);

        let routing_metric = if curr.Ipv4Metric > 0 {
            curr.Ipv4Metric
        } else if curr.Ipv6Metric > 0 {
            curr.Ipv6Metric
        } else {
            u32::MAX
        };

        let interface_luid = unsafe { curr.Luid.Value };
        let wifi = if interface_type == crate::export::InterfaceType::Wifi && is_up {
            query_active_wifi_diagnostics_for_luid(Some(interface_luid))
        } else {
            None
        };

        raw_adapters.push(RawAdapterInfo {
            name,
            description,
            friendly_name,
            interface_type,
            is_physical,
            is_up,
            routing_metric,
            link_speed_bps,
            mac_address,
            ipv4_addresses,
            ipv6_addresses,
            default_gateways,
            dns_servers,
            dhcp_enabled,
            dhcp_server,
            mtu,
            wifi,
        });

        curr_ptr = curr.Next;
    }

    raw_adapters
}

pub fn query_active_wifi_diagnostics() -> Option<crate::export::WifiDiagnostics> {
    query_active_wifi_diagnostics_for_luid(None)
}

pub fn query_active_wifi_diagnostics_for_luid(
    target_luid: Option<u64>,
) -> Option<crate::export::WifiDiagnostics> {
    unsafe {
        let mut negotiated = 0u32;
        let mut handle = 0isize;
        if WlanOpenHandle(2, std::ptr::null_mut(), &mut negotiated, &mut handle) != 0 || handle == 0
        {
            return None;
        }

        let mut list_ptr: *mut WLAN_INTERFACE_INFO_LIST = std::ptr::null_mut();
        let enum_res = WlanEnumInterfaces(handle, std::ptr::null_mut(), &mut list_ptr);
        if enum_res != 0 || list_ptr.is_null() {
            let _ = WlanCloseHandle(handle, std::ptr::null_mut());
            return None;
        }

        let mut found_diag: Option<crate::export::WifiDiagnostics> = None;
        let count = (*list_ptr).dwNumberOfItems;
        if count > 0 {
            let interfaces =
                std::slice::from_raw_parts((*list_ptr).InterfaceInfo.as_ptr(), count as usize);
            for iface in interfaces {
                if iface.isState == 1
                    && target_luid.is_none_or(|target| {
                        luid_for_wifi_guid(&iface.InterfaceGuid) == Some(target)
                    })
                {
                    let mut data_size = 0u32;
                    let mut data_ptr: *mut core::ffi::c_void = std::ptr::null_mut();

                    let mut ssid = String::new();
                    let mut bssid = String::new();
                    let mut fallback_phy = 0u32;
                    let mut fallback_quality = 0u32;
                    let mut fallback_rx_rate = 0u32;
                    let mut fallback_tx_rate = 0u32;

                    let query_conn = WlanQueryInterface(
                        handle,
                        &iface.InterfaceGuid,
                        WLAN_INTF_OPCODE_CURRENT_CONNECTION,
                        std::ptr::null_mut(),
                        &mut data_size,
                        &mut data_ptr,
                        std::ptr::null_mut(),
                    );

                    if query_conn == 0 && !data_ptr.is_null() {
                        let conn_attrs = &*(data_ptr as *const WLAN_CONNECTION_ATTRIBUTES);
                        let ssid_len =
                            conn_attrs.wlanAssociationAttributes.dot11Ssid.uSSIDLength as usize;
                        if ssid_len > 0 && ssid_len <= 32 {
                            let bytes =
                                &conn_attrs.wlanAssociationAttributes.dot11Ssid.ucSSID[..ssid_len];
                            let ssid_lossy = String::from_utf8_lossy(bytes);
                            let trimmed = ssid_lossy.trim_matches(['\0', ' ']);
                            if !trimmed.is_empty() {
                                ssid = trimmed.to_string();
                            }
                        }
                        if ssid.is_empty() {
                            let prof = wchar_to_string(&conn_attrs.strProfileName);
                            let trimmed = prof.trim();
                            if !trimmed.is_empty() {
                                ssid = trimmed.to_string();
                            }
                        }

                        let b = conn_attrs.wlanAssociationAttributes.dot11Bssid;
                        bssid = format!(
                            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
                            b[0], b[1], b[2], b[3], b[4], b[5]
                        );

                        fallback_phy = conn_attrs.wlanAssociationAttributes.dot11PhyType;
                        fallback_quality = conn_attrs.wlanAssociationAttributes.wlanSignalQuality;
                        fallback_rx_rate = conn_attrs.wlanAssociationAttributes.ulRxRate;
                        fallback_tx_rate = conn_attrs.wlanAssociationAttributes.ulTxRate;
                        WlanFreeMemory(data_ptr);
                    }

                    if ssid.is_empty() {
                        ssid = "Wi-Fi".to_string();
                    }

                    // Query realtime connection quality (OpCode 19)
                    let mut rt_size = 0u32;
                    let mut rt_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
                    let query_rt = WlanQueryInterface(
                        handle,
                        &iface.InterfaceGuid,
                        WLAN_INTF_OPCODE_REALTIME_CONNECTION_QUALITY,
                        std::ptr::null_mut(),
                        &mut rt_size,
                        &mut rt_ptr,
                        std::ptr::null_mut(),
                    );

                    if query_rt == 0 && !rt_ptr.is_null() {
                        let rt = &*(rt_ptr as *const WLAN_REALTIME_CONNECTION_QUALITY);
                        let quality_pct = rt.ulLinkQuality.min(100) as u8;

                        let (freq_mhz, rssi_dbm, ch_width) = if rt.ulNumLinks > 0 {
                            let link = &rt.linksInfo[0];
                            let r = if link.lRssi != 0 {
                                link.lRssi
                            } else {
                                signal_quality_to_rssi_dbm(quality_pct) as i32
                            };
                            let w = if link.ulBandwidth > 0 {
                                link.ulBandwidth
                            } else {
                                20
                            };
                            (link.ulChannelCenterFrequencyMhz, r, w)
                        } else {
                            (0, signal_quality_to_rssi_dbm(quality_pct) as i32, 20)
                        };

                        let band = wifi_band(freq_mhz, None);
                        let wifi_gen = wifi_generation_with_band(rt.dot11PhyType, band);
                        let tx_bps = if rt.ulTxRate > 0 {
                            (rt.ulTxRate as u64) * 1000
                        } else {
                            0
                        };
                        let rx_bps = if rt.ulRxRate > 0 {
                            (rt.ulRxRate as u64) * 1000
                        } else {
                            0
                        };

                        WlanFreeMemory(rt_ptr);

                        found_diag = Some(crate::export::WifiDiagnostics {
                            ssid,
                            bssid,
                            generation: wifi_gen,
                            band_ghz: band.as_str().to_string(),
                            channel: crate::backend::wifi_channel_number(freq_mhz).unwrap_or(0),
                            channel_width_mhz: ch_width,
                            rssi_dbm,
                            transmit_rate_bps: tx_bps,
                            receive_rate_bps: rx_bps,
                        });
                    } else {
                        let quality_pct = fallback_quality.min(100) as u8;
                        let rssi = signal_quality_to_rssi_dbm(quality_pct) as i32;
                        let tx_bps = (fallback_tx_rate as u64) * 1000;
                        let rx_bps = (fallback_rx_rate as u64) * 1000;

                        // Query channel number (Opcode 8)
                        let mut ch_size = 0u32;
                        let mut ch_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
                        let mut channel_val = 0u32;
                        if WlanQueryInterface(
                            handle,
                            &iface.InterfaceGuid,
                            WLAN_INTF_OPCODE_CHANNEL_NUMBER,
                            std::ptr::null_mut(),
                            &mut ch_size,
                            &mut ch_ptr,
                            std::ptr::null_mut(),
                        ) == 0
                            && !ch_ptr.is_null()
                        {
                            channel_val = *(ch_ptr as *const u32);
                            WlanFreeMemory(ch_ptr);
                        }

                        let band = wifi_band(
                            0,
                            if channel_val > 0 {
                                Some(channel_val)
                            } else {
                                None
                            },
                        );
                        let wifi_gen = wifi_generation_with_band(fallback_phy, band);

                        found_diag = Some(crate::export::WifiDiagnostics {
                            ssid,
                            bssid,
                            generation: wifi_gen,
                            band_ghz: band.as_str().to_string(),
                            channel: channel_val,
                            channel_width_mhz: 20,
                            rssi_dbm: rssi,
                            transmit_rate_bps: tx_bps,
                            receive_rate_bps: rx_bps,
                        });
                    }

                    if found_diag.is_some() {
                        break;
                    }
                }
            }
        }

        WlanFreeMemory(list_ptr as *mut core::ffi::c_void);
        let _ = WlanCloseHandle(handle, std::ptr::null_mut());
        found_diag
    }
}

pub fn query_adapter_diagnostics() -> (
    Option<crate::export::AdapterDiagnostics>,
    Vec<crate::export::AdapterSummary>,
) {
    let raw = query_raw_adapters();
    select_active_adapter(raw)
}
