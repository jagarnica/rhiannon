use cpal;
use cpal::traits::*;

/// Enumerate all available input formats
pub fn list_inputs() {
    let device = cpal::default_host().default_input_device()
        .expect("no input device found");

    if let Ok(mut fmt_range) = device.supported_input_formats() {
        while let Some(fmt) = fmt_range.next() {
            println!("{:?}", fmt);
        }
    }
}

/// Get the result of available hosts and then list them. 
pub fn list_hosts(){
    let string_of_hosts: Vec<_> = cpal::available_hosts()
    .into_iter()  
    .collect();
    println!("Hosts Found: {:?}", string_of_hosts);
}
