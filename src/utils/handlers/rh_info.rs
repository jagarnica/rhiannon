use crate::audioproccesing::inputs as ap;
use crate::utils::cli::InfoOpts;
///* Processes the info command arguments. 
pub fn handle_info(verbose: bool, opts: InfoOpts){
    
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
        }
        _ => (),

    }
   
}