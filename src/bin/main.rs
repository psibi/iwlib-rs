use iwlib::*;

fn main() {
    let wireless_info = get_wireless_info("wlp0s20f3".to_string());
    println!("Wireless info: {:?}", wireless_info);
}
