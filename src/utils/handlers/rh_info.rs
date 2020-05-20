use crate::audioproccesing::inputs as ap;
use crate::utils::cli::InfoOpts;
///* Processes the info command arguments. 
pub fn handle_info(verbose: bool, opts: InfoOpts){
    let verbose_enabled = verbose;
    match (&opts) {
        inputs  =>{
          match opts.inputs {
            true =>{
                println!("listing inputs... ")
            },
            _ =>{}
          }
        },
        outputs =>{
            match opts.outputs {
                true =>{
                    println!("listing outputs")
                },
                _ =>{},
            }
        },
        defaults =>{
            match opts.defaults {
                true =>{
                    println!("Listing defaults");
                    list_defaults(verbose_enabled);
                },
                _ =>{},
            }
        }
        _ => (),

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