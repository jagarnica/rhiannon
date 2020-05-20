use std::env;
use structopt::StructOpt;
mod info;
mod start;
//* This processes the arguements passed from the cli
#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "The Fastest Streaming Discord Bot")]
pub struct Cli {
    #[structopt(short = "v", global = true, long = "verbose")]
    verbose: bool,
    #[structopt(subcommand)] 
    commands: Subcommands
}
#[derive(StructOpt, Debug, PartialEq)]
pub enum Subcommands {
    #[structopt(name = "start")]
    Start (StartOpts),
    #[structopt(name = "info")]
    Info (InfoOpts)
}
#[derive(StructOpt, Debug, PartialEq)]
pub struct InfoOpts {
    #[structopt(short = "i", long = "inputs")]
    inputs: bool,
    #[structopt(short = "o", long = "outputs")]
    outputs: bool,
    #[structopt(short = "d", long = "defaults")]
    defaults: bool,
    #[structopt(short = "a", long = "all")]
    all: bool,
}

#[derive(StructOpt, Debug, PartialEq)]
pub struct StartOpts {
    /// Port number to run the bot from
    #[structopt(short, long, default_value = "8008")]
    port: String,
}
//* Processes all the arguments passed in the from cli
pub fn parse_args(args: Cli){
    println!("Args found {:?}", &args);
    let verbose_enabled = args.verbose;
    match &args {
        start=> {
            let opts = args.commands;
            match opts {
                start =>{
                    eprintln!("Opts found {:?}", start::Start)
                }
            }
            //handle_start(verbose_enabled, opts);
        },
        info=>{

        }
    }
}
//* handle the info command 
pub fn handle_info(){

}
//* handle the start command 
pub fn handle_start(verbose: bool, args: StartOpts){
    let verbose_enabled = verbose;
    
}

