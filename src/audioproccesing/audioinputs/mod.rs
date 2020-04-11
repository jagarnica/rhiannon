use cpal;
use cpal::traits::*;

pub fn list_inputs() {
    // enumerate all available input formats
    let device = cpal::default_host().default_input_device()
        .expect("no input device found");

    if let Ok(mut fmt_range) = device.supported_input_formats() {
        while let Some(fmt) = fmt_range.next() {
            println!("{:?}", fmt);
        }
    }
}

pub fn list_hosts(){
    // Get the result of available hosts and then list them. 
    let string_of_hosts: Vec<_> = cpal::available_hosts()
    .into_iter()  
    .collect();
    println!("Hosts Found: {:?}", string_of_hosts);
}
