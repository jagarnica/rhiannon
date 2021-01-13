#![allow(unused)] // don't warn on unused things
use cpal;
use cpal::traits::*;
mod utils;
mod audioproccesing;
use std::env;
use structopt::StructOpt;
use utils::cli as command;

fn main() {
    // enumerate all available input formats
    use audioproccesing::inputs as audioproc;
    
   //  audioproccesing::inputs::enumerate_device_info();
    //audioproccesing::inputs::list_default_input_formats();
    //audioproccesing::inputs::list_available_devices();
    //audioproccesing::inputs::list_default_input_formats();
    //audioproccesing::inputs::list_default_output_device();

    //audioproccesing::inputs::list_hosts();
    //audioproccesing::inputs::list_default_input_device();
    //audioproccesing::inputs::feedback();
    //audioproccesing::inputs::list_available_devices();

    // First get the arguments
    let commands = command::Cli::from_args();
    // Let cli process the arguments 
    command::parse_args(commands);
    
}


