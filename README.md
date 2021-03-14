# iwlib

[![CI](https://github.com/psibi/iwlib-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/psibi/iwlib-rs/actions/workflows/ci.yml)

Provides safe bindings on top of libiw library.

## Usage

Currently it exposes minimal interfaces, any additional contributions
welcome.

You can get the essid and it's quality using this:

``` rust
use iwlib::*;

fn main() {
    let wireless_info = get_wireless_info("wlp0s20f3".to_string());
    println!("Wireless info: {:?}", wireless_info);
}
```

Executing the above code on my network gives me this:

``` shellsession
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/iwlib-helper`
Wireless info: Some(WirelessInfo { wi_essid: "ichigokurasaki_5hz", wi_quality: 50 })
```
