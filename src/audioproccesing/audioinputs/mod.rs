use cpal;
use cpal::traits::*;

pub fn list_inputs() {
    let _host = cpal::default_host();
    // enumerate all available input formats
    let device = _host.default_input_device()
        .expect("no input device found");

    if let Ok(mut fmt_range) = device.supported_input_formats() {
        while let Some(fmt) = fmt_range.next() {
            println!("{:?}", fmt);
        }
    }
}

pub fn list_hosts(){
    let _hosts_found = cpal::available_hosts();
    // Get the result of available hosts and then list them. 
    let _string_of_hosts: Vec<_> = _hosts_found
    .into_iter()  
    .collect();
    println!("Hosts Found: {:?}", _string_of_hosts);
}
