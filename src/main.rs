#![allow(unused)] // don't warn on unused things
use cpal;
use cpal::traits::*;
mod cli;
mod audioproccesing;
use std::env;
use structopt::StructOpt;
use cli::Cli as Cli;

fn main() {
    // enumerate all available input formats
    use audioproccesing::inputs as audioproc;
    
    // audioproccesing::inputs::enumerate_device_info();
    //audioproccesing::inputs::list_default_input_formats();
    //audioproccesing::inputs::list_available_devices();
    //audioproccesing::inputs::list_default_input_formats();
    //audioproccesing::inputs::list_default_output_device();

    //audioproccesing::inputs::list_hosts();
    //audioproccesing::inputs::list_default_input_device();
    //audioproccesing::inputs::feedback();
    //audioproccesing::inputs::list_available_devices();

   
    let cmds = Cli::from_args();
    let verbose_enabled = cmds.verbose;
    eprintln!("Verbose mode activated? {:?}", verbose_enabled);
    handle_subcommands(cmds, verbose_enabled);
    
}
fn handle_subcommands(cli: Cli, verbose: bool){
    if let Some(subcommand) = cli.commands{
        match subcommand {
            cli::Subcommands::Start(cfg) => {
                println!("handle start:  {:?}", cfg);
            },
            cli::Subcommands::Info(cfg)=>{
                println!("handle info:  {:?}", cfg);
               match cfg {
                   all  =>{
                     
                   },
               }

            }
           
        }
    }
}

