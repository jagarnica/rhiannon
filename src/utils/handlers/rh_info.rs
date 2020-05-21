use crate::audioproccesing::inputs as ap;
use crate::utils::cli::InfoOpts;
use crate::utils::cli as cli;
///* Processes the info command arguments. 
pub fn handle_info(verbose: bool, opts: InfoOpts){
    let verbose_enabled = verbose;

    // User wants us to list defaults for the system 
    if (opts.defaults) {
        println!("listing defaults... ")
    }
    // User wants us to list the output devices for the system 
    if (opts.outputs){
        println!("listing outputs...")
    }
    // List all the input devices for the system 
    if (opts.inputs){
        println!("listing inputs...")
    }
    // enumerate all the devices on the system 
    if (opts.all){
        list_audio_details();
    }
    
}
fn test_func(test_arg: bool){
    println!("Testing!!!")
}
// List the inputs founds for the device 
fn list_inputs(verbose: bool){
   //* TODO Finish listing inputs 
    println!("Todo...")
}

fn list_audio_details(){
    ap::enumerate_device_info();
}

fn list_defaults(verbose: bool){
    ap::list_default_input_device();
    ap::list_default_output_device();
}