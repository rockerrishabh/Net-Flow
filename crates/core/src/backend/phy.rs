use std::time::Instant;

use super::types::{
    EthernetLinkMetrics, InterfaceMedium, PhysicalLinkInfo, WifiPhyMetrics,
    signal_quality_to_rssi_dbm, wifi_band, wifi_channel_number, wifi_generation_with_band,
};
use super::wchar_to_string;

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct DOT11_SSID {
    pub uSSIDLength: u32,
    pub ucSSID: [u8; 32],
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct WLAN_ASSOCIATION_ATTRIBUTES {
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: u32,
    pub dot11Bssid: [u8; 6],
    pub dot11PhyType: u32,
    pub uDot11PhyIndex: u32,
    pub wlanSignalQuality: u32,
    pub ulRxRate: u32,
    pub ulTxRate: u32,
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct WLAN_SECURITY_ATTRIBUTES {
    pub bSecurityEnabled: i32,
    pub bOneXEnabled: i32,
    pub dot11AuthAlgorithm: u32,
    pub dot11CipherAlgorithm: u32,
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct WLAN_CONNECTION_ATTRIBUTES {
    pub isState: u32,
    pub wlanConnectionMode: u32,
    pub strProfileName: [u16; 256],
    pub wlanAssociationAttributes: WLAN_ASSOCIATION_ATTRIBUTES,
    pub wlanSecurityAttributes: WLAN_SECURITY_ATTRIBUTES,
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct WLAN_RATE_SET {
    pub uRateSetLength: u32,
    pub usRateSet: [u16; 126],
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO {
    pub ucLinkID: u8,
    pub ulChannelCenterFrequencyMhz: u32,
    pub ulBandwidth: u32,
    pub lRssi: i32,
    pub wlanRateSet: WLAN_RATE_SET,
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct WLAN_REALTIME_CONNECTION_QUALITY {
    pub dot11PhyType: u32,
    pub ulLinkQuality: u32,
    pub ulRxRate: u32,
    pub ulTxRate: u32,
    pub bIsMLOConnection: i32,
    pub ulNumLinks: u32,
    pub linksInfo: [WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO; 1],
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct WLAN_INTERFACE_INFO {
    pub InterfaceGuid: [u8; 16],
    pub strInterfaceDescription: [u16; 256],
    pub isState: u32,
}

#[allow(non_snake_case, clippy::upper_case_acronyms)]
#[repr(C)]
pub(crate) struct WLAN_INTERFACE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub InterfaceInfo: [WLAN_INTERFACE_INFO; 1],
}

pub(crate) const WLAN_INTF_OPCODE_CURRENT_CONNECTION: u32 = 7;
pub(crate) const WLAN_INTF_OPCODE_CHANNEL_NUMBER: u32 = 8;
pub(crate) const WLAN_INTF_OPCODE_REALTIME_CONNECTION_QUALITY: u32 = 19;

pub(crate) fn luid_for_wifi_guid(guid_bytes: &[u8; 16]) -> Option<u64> {
    use windows::Win32::NetworkManagement::{
        IpHelper::ConvertInterfaceGuidToLuid, Ndis::NET_LUID_LH,
    };

    let guid = windows::core::GUID::from_values(
        u32::from_le_bytes(guid_bytes[0..4].try_into().ok()?),
        u16::from_le_bytes(guid_bytes[4..6].try_into().ok()?),
        u16::from_le_bytes(guid_bytes[6..8].try_into().ok()?),
        guid_bytes[8..16].try_into().ok()?,
    );
    let mut luid = NET_LUID_LH { Value: 0 };
    if unsafe { ConvertInterfaceGuidToLuid(&guid, &mut luid) }.0 != 0 {
        return None;
    }
    Some(unsafe { luid.Value })
}

#[link(name = "wlanapi")]
unsafe extern "system" {
    pub(crate) fn WlanOpenHandle(
        dwClientVersion: u32,
        pReserved: *mut core::ffi::c_void,
        pdwNegotiatedVersion: *mut u32,
        phClientHandle: *mut isize,
    ) -> u32;
    pub(crate) fn WlanCloseHandle(hClientHandle: isize, pReserved: *mut core::ffi::c_void) -> u32;
    pub(crate) fn WlanEnumInterfaces(
        hClientHandle: isize,
        pReserved: *mut core::ffi::c_void,
        ppInterfaceList: *mut *mut WLAN_INTERFACE_INFO_LIST,
    ) -> u32;
    pub(crate) fn WlanQueryInterface(
        hClientHandle: isize,
        pInterfaceGuid: *const [u8; 16],
        OpCode: u32,
        pReserved: *mut core::ffi::c_void,
        pdwDataSize: *mut u32,
        ppData: *mut *mut core::ffi::c_void,
        pWlanOpcodeValueType: *mut u32,
    ) -> u32;
    pub(crate) fn WlanFreeMemory(pMemory: *mut core::ffi::c_void);
}

/// Queries deep Wi-Fi physical layer (PHY) telemetry using Windows Native Wi-Fi API.
/// Uses `WLAN_REALTIME_CONNECTION_QUALITY` as the primary rate, quality, and MLO source
/// without requiring Windows location permissions, with graceful fallback to connection attributes.
pub fn query_active_wifi_metrics() -> Option<WifiPhyMetrics> {
    query_active_wifi_metrics_for_luid(None)
}

pub fn query_active_wifi_metrics_for_luid(target_luid: Option<u64>) -> Option<WifiPhyMetrics> {
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

        let mut found_metrics: Option<WifiPhyMetrics> = None;
        let count = (*list_ptr).dwNumberOfItems;
        if count > 0 {
            let interfaces =
                std::slice::from_raw_parts((*list_ptr).InterfaceInfo.as_ptr(), count as usize);
            for iface in interfaces {
                // wlan_interface_state_connected = 1
                if iface.isState == 1
                    && target_luid.is_none_or(|target| {
                        luid_for_wifi_guid(&iface.InterfaceGuid) == Some(target)
                    })
                {
                    let mut data_size = 0u32;
                    let mut data_ptr: *mut core::ffi::c_void = std::ptr::null_mut();

                    // 1. Query current connection to extract SSID
                    let mut ssid = String::new();
                    let query_conn = WlanQueryInterface(
                        handle,
                        &iface.InterfaceGuid,
                        WLAN_INTF_OPCODE_CURRENT_CONNECTION,
                        std::ptr::null_mut(),
                        &mut data_size,
                        &mut data_ptr,
                        std::ptr::null_mut(),
                    );
                    let mut fallback_phy = 0u32;
                    let mut fallback_quality = 0u32;
                    let mut fallback_rx_rate = 0u32;
                    let mut fallback_tx_rate = 0u32;

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
                        fallback_phy = conn_attrs.wlanAssociationAttributes.dot11PhyType;
                        fallback_quality = conn_attrs.wlanAssociationAttributes.wlanSignalQuality;
                        fallback_rx_rate = conn_attrs.wlanAssociationAttributes.ulRxRate;
                        fallback_tx_rate = conn_attrs.wlanAssociationAttributes.ulTxRate;
                        WlanFreeMemory(data_ptr);
                    }

                    if ssid.is_empty() {
                        ssid = "Wi-Fi".to_string();
                    }

                    // 2. Query WLAN_REALTIME_CONNECTION_QUALITY (Opcode 19)
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
                        let is_mlo = rt.bIsMLOConnection != 0;
                        let link_count = rt.ulNumLinks.clamp(1, 255) as u8;

                        let (freq_mhz, rssi_dbm) = if rt.ulNumLinks > 0 {
                            let link = &rt.linksInfo[0];
                            let r = if link.lRssi != 0 {
                                Some(link.lRssi as i16)
                            } else {
                                Some(signal_quality_to_rssi_dbm(quality_pct))
                            };
                            (link.ulChannelCenterFrequencyMhz, r)
                        } else {
                            (0, Some(signal_quality_to_rssi_dbm(quality_pct)))
                        };

                        let band = wifi_band(freq_mhz, None);
                        let wifi_gen = wifi_generation_with_band(rt.dot11PhyType, band);
                        let tx_mbps = if rt.ulTxRate > 0 {
                            Some(rt.ulTxRate / 1000)
                        } else {
                            None
                        };
                        let rx_mbps = if rt.ulRxRate > 0 {
                            Some(rt.ulRxRate / 1000)
                        } else {
                            None
                        };

                        WlanFreeMemory(rt_ptr);

                        found_metrics = Some(WifiPhyMetrics {
                            ssid,
                            generation: wifi_gen,
                            band,
                            channel: wifi_channel_number(freq_mhz),
                            signal_quality_pct: quality_pct,
                            rssi_dbm,
                            tx_rate_mbps: tx_mbps,
                            rx_rate_mbps: rx_mbps,
                            is_mlo,
                            link_count,
                        });
                    } else {
                        // 3. Fallback to association attributes + channel query
                        let quality_pct = fallback_quality.min(100) as u8;
                        let rssi = Some(signal_quality_to_rssi_dbm(quality_pct));
                        let tx_mbps = if fallback_tx_rate > 0 {
                            Some(fallback_tx_rate / 1000)
                        } else {
                            None
                        };
                        let rx_mbps = if fallback_rx_rate > 0 {
                            Some(fallback_rx_rate / 1000)
                        } else {
                            None
                        };

                        // Query channel number (Opcode 8)
                        let mut ch_size = 0u32;
                        let mut ch_ptr: *mut core::ffi::c_void = std::ptr::null_mut();
                        let mut channel_opt: Option<u32> = None;
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
                            let ch = *(ch_ptr as *const u32);
                            if ch > 0 {
                                channel_opt = Some(ch);
                            }
                            WlanFreeMemory(ch_ptr);
                        }

                        let band = wifi_band(0, channel_opt);
                        let wifi_gen = wifi_generation_with_band(fallback_phy, band);

                        found_metrics = Some(WifiPhyMetrics {
                            ssid,
                            generation: wifi_gen,
                            band,
                            channel: channel_opt,
                            signal_quality_pct: quality_pct,
                            rssi_dbm: rssi,
                            tx_rate_mbps: tx_mbps,
                            rx_rate_mbps: rx_mbps,
                            is_mlo: false,
                            link_count: 1,
                        });
                    }

                    if found_metrics.is_some() {
                        break;
                    }
                }
            }
        }

        WlanFreeMemory(list_ptr as *mut core::ffi::c_void);
        let _ = WlanCloseHandle(handle, std::ptr::null_mut());
        found_metrics
    }
}

static WIFI_PHY_CACHE: std::sync::Mutex<(Option<u64>, Option<WifiPhyMetrics>, Option<Instant>)> =
    std::sync::Mutex::new((None, None, None));

/// Cached Wi-Fi PHY metrics lookup with a 2-second TTL to avoid spamming WlanAPI on every 500ms tick.
pub fn query_cached_wifi_phy() -> Option<WifiPhyMetrics> {
    query_cached_wifi_phy_for_luid(None)
}

pub fn query_cached_wifi_phy_for_luid(target_luid: Option<u64>) -> Option<WifiPhyMetrics> {
    const CACHE_TTL: std::time::Duration = std::time::Duration::from_secs(2);
    let now = Instant::now();
    if let Ok(mut cache) = WIFI_PHY_CACHE.lock() {
        if cache.0 == target_luid
            && let Some(last_query) = cache.2
            && now.duration_since(last_query) < CACHE_TTL
        {
            return cache.1.clone();
        }
        let fresh = query_active_wifi_metrics_for_luid(target_luid);
        *cache = (target_luid, fresh.clone(), Some(now));
        fresh
    } else {
        query_active_wifi_metrics_for_luid(target_luid)
    }
}

/// Cached Wi-Fi SSID lookup utilizing the unified Wi-Fi cache.
pub fn query_cached_wifi_ssid() -> Option<String> {
    query_cached_wifi_phy().map(|w| w.ssid)
}

/// Query active Ethernet connection link speed using IP Helper MIB_IF_ROW2.
pub fn query_active_ethernet_metrics() -> Option<EthernetLinkMetrics> {
    query_active_ethernet_metrics_for_luid(None)
}

pub fn query_active_ethernet_metrics_for_luid(
    target_luid: Option<u64>,
) -> Option<EthernetLinkMetrics> {
    use windows::Win32::NetworkManagement::IpHelper::{FreeMibTable, GetIfTable2, MIB_IF_TABLE2};

    unsafe {
        let mut table: *mut MIB_IF_TABLE2 = std::ptr::null_mut();
        if GetIfTable2(&mut table).0 != 0 || table.is_null() {
            return None;
        }

        let num_entries = (*table).NumEntries as usize;
        let rows = std::slice::from_raw_parts((*table).Table.as_ptr(), num_entries);
        let mut best: Option<EthernetLinkMetrics> = None;

        for row in rows {
            // IF_TYPE_ETHERNET_CSMACD = 6, IF_TYPE_GIGABITETHERNET = 117, IF_TYPE_FASTETHER = 62, IfOperStatusUp = 1
            if (row.Type == 6 || row.Type == 117 || row.Type == 62)
                && row.OperStatus.0 == 1
                && target_luid.is_none_or(|target| row.InterfaceLuid.Value == target)
            {
                let tx_bps = row.TransmitLinkSpeed;
                let rx_bps = row.ReceiveLinkSpeed;
                let desc = wchar_to_string(&row.Description);
                let alias = wchar_to_string(&row.Alias);
                let name = if !alias.is_empty() {
                    alias
                } else if !desc.is_empty() {
                    desc
                } else {
                    "Ethernet".to_string()
                };

                best = Some(EthernetLinkMetrics {
                    adapter_name: name,
                    tx_speed_bps: tx_bps,
                    rx_speed_bps: rx_bps,
                });
                break;
            }
        }

        FreeMibTable(table as *const core::ffi::c_void);
        best
    }
}

/// Resolves physical layer link information according to active medium.
pub fn query_physical_link_info(primary_medium: InterfaceMedium) -> Option<PhysicalLinkInfo> {
    query_physical_link_info_for_luid(primary_medium, None)
}

pub fn query_physical_link_info_for_luid(
    primary_medium: InterfaceMedium,
    primary_luid: Option<u64>,
) -> Option<PhysicalLinkInfo> {
    match primary_medium {
        InterfaceMedium::Wifi => {
            query_cached_wifi_phy_for_luid(primary_luid).map(PhysicalLinkInfo::Wifi)
        }
        InterfaceMedium::Ethernet => {
            query_active_ethernet_metrics_for_luid(primary_luid).map(PhysicalLinkInfo::Ethernet)
        }
        _ => None,
    }
}
