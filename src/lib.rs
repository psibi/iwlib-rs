use iwlib_sys::*;
use std::ffi::CStr;
use std::ffi::CString;

/// Represents wireless information for a particular ESSID
#[derive(PartialEq, PartialOrd, Debug)]
pub struct WirelessInfo {
    /// ESSID  identifying the name of the wireless network
    pub wi_essid: String,
    /// Quality of the wireless network
    pub wi_quality: u8,
}

/// Get current active `WirelessInfo` for the `interface`.
///
/// # Arguments
///
/// * `interface` - String representing the name name of the network
/// interface for your wireless hardware. In Linux systems, you can
/// find that out using `iw dev` command.
///
/// # Examples
///
/// ```
/// use iwlib::*;
/// let wireless_info = get_wireless_info("wlan0".to_string());
/// ```
pub fn get_wireless_info(interface: String) -> Option<WirelessInfo> {
    let interface_name = CString::new(interface).unwrap();
    let mut config: wireless_config = Default::default();
    let mut statistics: iw_statistics = Default::default();
    let mut range: iw_range = Default::default();
    let handle = unsafe { iw_sockets_open() };
    let basic_config_status =
        unsafe { iw_get_basic_config(handle, interface_name.as_ptr(), &mut config) };
    if basic_config_status < 0 {
        unsafe {
            libc::close(handle);
        }
        return None;
    }
    let stats_status =
        unsafe { iw_get_stats(handle, interface_name.as_ptr(), &mut statistics, &range, 1) };
    let range_status = unsafe { iw_get_range_info(handle, interface_name.as_ptr(), &mut range) };
    unsafe {
        libc::close(handle);
    }
    let mut quality: f64 = 0.0;
    if stats_status >= 0 && range_status >= 0 {
        let stats_quality = get_quality(statistics.qual);
        let range_quality = get_quality(range.max_qual);
        if range_quality != 0 {
            quality = stats_quality as f64 / range_quality as f64;
        }
    }

    compute_essid(config).map(|essid| WirelessInfo {
        wi_essid: essid,
        wi_quality: (quality * 100.0) as u8,
    })
}

fn compute_essid(wconfig: wireless_config) -> Option<String> {
    if wconfig.has_essid != 0 && wconfig.essid_on != 0 {
        let essid = unsafe { CStr::from_ptr(wconfig.essid.as_ptr()) };
        return Some(essid.to_owned().into_string().unwrap());
    }
    None
}

fn get_quality(config: iw_quality) -> u8 {
    config.qual
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_crash() {
        let wireless_info = get_wireless_info("wlp0s20f3".to_string());
        println!("Wireless info: {:?}", wireless_info);
    }
}
