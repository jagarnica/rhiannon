use cpal;
use cpal::traits::*;

pub fn listinputs() {
    let host = cpal::default_host();
    // enumerate all available input formats
    let device = host.default_input_device()
        .expect("no input device found");

    if let Ok(mut fmt_range) = device.supported_input_formats() {
        while let Some(fmt) = fmt_range.next() {
            println!("{:?}", fmt);
        }
    }
}
