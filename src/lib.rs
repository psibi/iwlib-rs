#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(clippy::all)]

use std::ffi::CStr;
use std::ffi::CString;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[derive(PartialEq, PartialOrd, Debug)]
pub struct WirelessInfo {
    wiEssid: String,
    wiQuality: u8,
}

pub fn get_wireless_info(interface: String) -> Option<WirelessInfo> {
    let interface_name = CString::new(interface).unwrap();

    unsafe {
        let mut config: wireless_config = Default::default();
        let mut statistics: iw_statistics = Default::default();
        let mut range: iw_range = Default::default();
        let handle = iw_sockets_open();

        let bcr = iw_get_basic_config(handle, interface_name.as_ptr(), &mut config);
        let str = iw_get_stats(
            handle,
            interface_name.as_ptr(),
            &mut statistics,
            &mut range,
            1,
        );
        let rgr = iw_get_range_info(handle, interface_name.as_ptr(), &mut range);
        if bcr < 0 {
            return None;
        }
        let mut quality: f64 = 0.0;
        if str >= 0 && rgr >= 0 {
            let stats_quality = compute_quality(statistics.qual);
            let range_quality = compute_quality(range.max_qual);
            if range_quality != 0 {
                quality = stats_quality as f64 / range_quality as f64;
            }
        }
        match compute_essid(config) {
            None => return None,
            Some(essid) => {
                return Some(WirelessInfo {
                    wiEssid: essid,
                    wiQuality: (quality * 100.0) as u8,
                })
            }
        }
    }
}

fn compute_essid(wconfig: wireless_config) -> Option<String> {
    if wconfig.has_essid != 0 && wconfig.essid_on != 0 {
        let essid = unsafe { CStr::from_ptr(wconfig.essid.as_ptr()) };
        return Some(essid.to_owned().into_string().unwrap());
    }
    None
}

fn compute_quality(config: iw_quality) -> u8 {
    config.qual
}

// todo: drop memory

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_does_not_crash() {
        let wireless_info = get_wireless_info("wlp0s20f3".to_string());
        println!("Wireless info: {:?}", wireless_info);
    }
}
