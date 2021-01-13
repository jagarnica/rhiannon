use crate::audioproccesing::inputs as ap;
use crate::utils::cli;
use crate::utils::cli::InfoOpts;

/// Processes the info command arguments.
pub fn handle_info(verbose: bool, opts: InfoOpts) {
    let verbose_enabled = verbose;
  
    // Handle there being no arguments 
    if opts.empty() {
        handle_no_args();
    }
    if (opts.help) {
        opts.show_help();
    }
    // User wants us to list defaults for the system
    if (opts.defaults) {
        list_defaults(&verbose_enabled);
    }
    // User wants us to list the output devices for the system
    if (opts.outputs) {
        println!("listing outputs...");
    }
    // List all the input devices for the system
    if (opts.inputs) {
        list_inputs(&verbose_enabled);
    }
    // enumerate all the devices on the system
    if (opts.all) {
        list_audio_details();
    }
  
}



// List the inputs founds for the device
fn list_inputs(verbose: &bool) {
    //* TODO Finish listing inputs
    println!("Uhh I can't list inputs yet...")
}

fn list_audio_details() {
    ap::enumerate_device_info();
}

fn list_defaults(verbose: &bool) {
    ap::list_default_input_device();
    ap::list_default_output_device();
}

fn handle_no_args() {
    println!("Please provide arguments for what you would like info on. Try -h for help.");
}