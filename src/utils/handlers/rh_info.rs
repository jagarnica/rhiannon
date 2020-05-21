use crate::audioproccesing::inputs as ap;
use crate::utils::cli::InfoOpts;
///* Processes the info command arguments. 
pub fn handle_info(verbose: bool, opts: InfoOpts){
    let verbose_enabled = verbose;
    println!("Howdy! here the options found {:?}", &opts);
    match opts.defaults {
        true =>{
            // User wants us to list the defaults 
            println!("listing defaults... ")
        }
        _=> {}
    }
    match opts.outputs {
        true =>{
            // User wants us to list the outputs 
            println!("listing outputs...")
        },
        _ =>{}
    }
    match opts.inputs {
        true => {
            println!("User wants use to list the inputs...")
        },
        _ =>{}
    }
    match opts.all  {
        true => {
            println!("Printing out all details... ");
        },
        _ =>{}
    }
   
}
// List the inputs founds for the device 
fn list_inputs(verbose: bool){
   //* TODO Finish listing inputs 
    println!("Todo...")
}

fn list_defaults(verbose: bool){
    ap::list_default_input_device();
    ap::list_default_output_device();
}