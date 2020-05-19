#![allow(unused)] // don't warn on unused things
use cpal;
use cpal::traits::*;
mod cli;
mod audioproccesing;
use std::env;
use structopt::StructOpt;

fn main() {
    // enumerate all available input formats
    use audioproccesing::inputs as audioproc;
    use cli::Cli as Cli;
    // audioproccesing::inputs::enumerate_device_info();
    //audioproccesing::inputs::list_default_input_formats();
    //audioproccesing::inputs::list_available_devices();
    //audioproccesing::inputs::list_default_input_formats();
    //audioproccesing::inputs::list_default_output_device();

    //audioproccesing::inputs::list_hosts();
    //audioproccesing::inputs::list_default_input_device();
    //audioproccesing::inputs::feedback();
    //audioproccesing::inputs::list_available_devices();


    match Cli::from_args() {
        Cli::Start { verbose, opts } => {
            println!("Still WIP");
        },
        Cli::Info { opts } =>{
            let mut verbose_enabled = false;
            println!("Options found {:?}", &opts);
          match (opts,true) {
              verbose =>{
                
                println!("Verbose mode activated");
                verbose_enabled = true;
                }
                
              
              all =>{
                /// List all the device information we have
                println!("printing out all info...");
                audioproc::enumerate_device_info();
              },
              inputs=>{
                //  Display the inputs 
                audioproc::list_available_devices();
              },

              _ => {}
          }          
        }

        _ => (),
    }
    
}

