
use cpal::*;
use cpal::traits::*;
use rodio::*;
use anyhow;
use ringbuf::RingBuffer;
/// prints a list of the accepted input formats for the default device
pub fn list_default_input_formats() {
    let input_device = rodio::default_input_device().unwrap();
    println!("Default input formats for device : {:?}",input_device.name());
    if let Ok(mut fmt_range) = input_device.supported_input_formats() {
        while let Some(fmt) = fmt_range.next(){
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


//* Prints out the the name of the default output device. 
pub fn list_default_output_device() -> Result<(), anyhow::Error> {
    let default_device = rodio::default_output_device().expect("No default output device found.");
    println!("Default output device: {:?}", default_device.name()?);
    Ok(())
}

pub fn enumerate_device_info() -> Result<(), anyhow::Error>{
    println!("Supported hosts:\n  {:?}", cpal::ALL_HOSTS);
    let available_hosts = cpal::available_hosts();
    println!("Available hosts:\n  {:?}", available_hosts);

    for host_id in available_hosts {
        println!("{}", host_id.name());
        let host = cpal::host_from_id(host_id)?;
        let default_in = host.default_input_device().map(|e| e.name().unwrap());
        let default_out = host.default_output_device().map(|e| e.name().unwrap());
        println!("  Default Input Device:\n    {:?}", default_in);
        println!("  Default Output Device:\n    {:?}", default_out);

        let devices = host.devices()?;
        println!("  Devices: ");
        for (device_index, device) in devices.enumerate() {
            println!("  {}. \"{}\"", device_index + 1, device.name()?);

            // Input configs
            if let Ok(conf) = device.default_input_format() {
                println!("    Default input stream config:\n      {:?}", conf);
            }
            let mut input_configs = match device.supported_input_formats() {
                Ok(f) => f.peekable(),
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            };
            if input_configs.peek().is_some() {
                println!("    All supported input stream configs:");
                for (config_index, config) in input_configs.enumerate() {
                    println!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    );
                }
            }

            // Output configs
            if let Ok(conf) = device.default_output_format() {
                println!("    Default output stream config:\n      {:?}", conf);
            }
            let mut output_configs = match device.supported_output_formats() {
                Ok(f) => f.peekable(),
                Err(e) => {
                    println!("Error: {:?}", e);
                    continue;
                }
            };
            if output_configs.peek().is_some() {
                println!("    All supported output stream configs:");
                for (config_index, config) in output_configs.enumerate() {
                    println!(
                        "      {}.{}. {:?}",
                        device_index + 1,
                        config_index + 1,
                        config
                    );
                }
            }
        }
    }

    Ok(())
}

/// Prints out a list of possible devices found on the current default host 
pub fn list_available_devices() -> Result<(), anyhow::Error>{
    let devices = rodio::devices()?;
    print!("  Devices: \n");
    for (device_index, device) in devices.enumerate() {
        println!("  {}. \"{}\"", device_index + 1, device.name()?);
    }

    Ok(())
}
