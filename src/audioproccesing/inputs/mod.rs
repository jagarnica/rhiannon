use anyhow;
use cpal::traits::*;
use cpal::*;
use portaudio as pa;
use ringbuf::RingBuffer;
use rodio::*;
use std::io;
const INTERLEAVED: bool = true;
const LATENCY: pa::Time = 0.0; // Ignored by PortAudio::is_*_format_supported.
const STANDARD_SAMPLE_RATES: [f64; 13] = [
    8000.0, 9600.0, 11025.0, 12000.0, 16000.0, 22050.0, 24000.0, 32000.0, 44100.0, 48000.0,
    88200.0, 96000.0, 192000.0,
];

/// prints a list of the accepted input formats for the default device
pub fn list_default_input_formats() {
    let input_device = rodio::default_input_device().unwrap();
    println!(
        "Default input formats for device : {:?}",
        input_device.name()
    );
    if let Ok(mut fmt_range) = input_device.supported_input_formats() {
        while let Some(fmt) = fmt_range.next() {
            println!("{:?}", fmt);
        }
    }
}

/// Get the result of available hosts and then list them.
pub fn list_hosts() {
    let string_of_hosts: Vec<_> = cpal::available_hosts().into_iter().collect();
    println!("Hosts Found: {:?}", string_of_hosts);
}

//* Prints out the the name of the default output device.
pub fn list_default_output_device() -> Result<(), anyhow::Error> {
    let pa = pa::PortAudio::new()?;
    let default_output = pa.default_output_device()?;
    let output_info = pa.device_info(default_output)?;
    println!("Default output device info: {:#?}", &output_info);

    Ok(())
}

pub fn enumerate_device_info() -> Result<(), anyhow::Error> {
    
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

/// Prints out a list of possible devices found on the current default host using PortAudio
pub fn list_available_devices() -> Result<(), anyhow::Error> {
    let pa = pa::PortAudio::new()?;
    println!("All devices:");
    for device in pa.devices()? {
        let (idx, info) = device?;
        println!("--------------------------------------- {:?}", idx);
        println!("{:#?}", &info);

        let in_channels = info.max_input_channels;
        let input_params = pa::StreamParameters::<i16>::new(idx, in_channels, INTERLEAVED, LATENCY);
        let out_channels = info.max_output_channels;
        let output_params =
            pa::StreamParameters::<i16>::new(idx, out_channels, INTERLEAVED, LATENCY);

        println!(
            "Supported standard sample rates for half-duplex 16-bit {} channel input:",
            in_channels
        );
        for &sample_rate in &STANDARD_SAMPLE_RATES {
            if pa
                .is_input_format_supported(input_params, sample_rate)
                .is_ok()
            {
                println!("\t{}hz", sample_rate);
            }
        }

        println!(
            "Supported standard sample rates for half-duplex 16-bit {} channel output:",
            out_channels
        );
        for &sample_rate in &STANDARD_SAMPLE_RATES {
            if pa
                .is_output_format_supported(output_params, sample_rate)
                .is_ok()
            {
                println!("\t{}hz", sample_rate);
            }
        }

        println!("Supported standard sample rates for full-duplex 16-bit {} channel input, {} channel output:",
                 in_channels, out_channels);
        for &sample_rate in &STANDARD_SAMPLE_RATES {
            if pa
                .is_duplex_format_supported(input_params, output_params, sample_rate)
                .is_ok()
            {
                println!("\t{}hz", sample_rate);
            }
        }
    }

    Ok(())
}
//* Prints out the default input device found
pub fn list_default_input_device() -> Result<(), anyhow::Error> {
    let pa = pa::PortAudio::new()?;
    let default_input = pa.default_input_device()?;
    let input_info = pa.device_info(default_input)?;
    println!("Default input device info: {:#?}", &input_info);

    Ok(())
}
/// Feeds back the audio from the default input device to the output device.
/// It is super useful for testing purposes.
pub fn feedback() -> Result<(), anyhow::Error> {
    let sample_rate: f64 = 44_100.0;
    let frame_size: u32 = 2048;
    let channels: i32 = 2;
    let interleaved: bool = true;

    let pa = pa::PortAudio::new()?;

    println!("PortAudio:");
    println!("version: {}", pa.version());
    println!("version text: {:?}", pa.version_text());
    println!("host count: {}", pa.host_api_count()?);

    // lets get the default host
    let default_host = pa.default_host_api()?;
    println!("default host: {:#?}", pa.host_api_info(default_host));

    // Get the default input device
    let def_input = pa.default_input_device()?;
    let input_info = pa.device_info(def_input)?;
    println!("Default input device info: {:#?}", &input_info);

    // Construct the input stream parameters.
    let latency = input_info.default_low_input_latency;
    let input_params = pa::StreamParameters::<f32>::new(def_input, channels, interleaved, latency);

    let def_output = pa.default_output_device()?;
    let output_info = pa.device_info(def_output)?;
    println!("Default output device info: {:#?}", &output_info);

    // Construct the output stream parameters.
    let latency = output_info.default_low_output_latency;
    let output_params = pa::StreamParameters::new(def_output, channels, interleaved, latency);
    // Check that the stream format is supported.
    pa.is_duplex_format_supported(input_params, output_params, sample_rate)?;

    // Construct the settings with which we'll open our duplex stream.
    let settings =
        pa::DuplexStreamSettings::new(input_params, output_params, sample_rate, frame_size);

    let mut user_input = String::new();
    let mut listening_active = true;

    // A callback to pass to the non-blocking stream.
    let callback = move |pa::DuplexStreamCallbackArgs {
                             in_buffer,
                             out_buffer,
                             frames,
                             time,
                             ..
                         }| {
        assert!(frames == frame_size as usize);

        // Pass the input straight to the output - BEWARE OF FEEDBACK!
        for (output_sample, input_sample) in out_buffer.iter_mut().zip(in_buffer.iter()) {
            *output_sample = *input_sample;
        }

        pa::Continue
    };

    // Construct a stream with input and output sample types of f32.
    let mut stream = pa.open_non_blocking_stream(settings, callback)?;
    println!("Type ''stop'' in order to no longer listen");
    stream.start()?;

    // Loop while the non-blocking stream is active.
    while let true = stream.is_active()? {
        // Do some stuff!

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        if user_input.trim().contains("stop") {
            println!("Exiting...");

            stream.stop()?;
        } else {
            // reset user input
            user_input = "".to_string();
        }
    }
    stream.stop()?;
    Ok(())
}
