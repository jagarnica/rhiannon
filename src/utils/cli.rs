use std::env;
use structopt::StructOpt;
use crate::utils::handlers::rh_start as rh_start;
use crate::utils::handlers::rh_info as rh_info;
//* This processes the arguements passed from the cli
#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "The Fastest Streaming Discord Bot")]
pub struct Cli {
    #[structopt(short = "v", global = true, long = "verbose")]
    verbose: bool,
    #[structopt(subcommand)] 
    commands: Option<Rhiannon>
}
#[derive(StructOpt, Debug, PartialEq)]
pub enum Rhiannon {
    #[structopt(name = "start")]
    Start (StartOpts),
    #[structopt(name = "info")]
    Info (InfoOpts)
}
#[derive(StructOpt, Debug, PartialEq)]
pub struct InfoOpts {
    #[structopt(short = "i", long = "inputs")]
    pub inputs: bool,
    #[structopt(short = "o", long = "outputs")]
    pub outputs: bool,
    #[structopt(short = "d", long = "defaults")]
    pub defaults: bool,
    #[structopt(short = "a", long = "all")]
    pub all: bool,
}

#[derive(StructOpt, Debug, PartialEq)]
pub struct StartOpts {
    /// Port number to run the bot from
    #[structopt(short, long, default_value = "8008")]
    port: String,
}
//* Processes all the arguments passed in the from cli
pub fn parse_args(args: Cli){
    let verbose_enabled = args.verbose;
     // handle subcommands
     if let Some(subcommand) = args.commands{
        match subcommand {
            Rhiannon::Start(cfg) => {
                rh_start::handle_start(verbose_enabled, cfg);
            },
            Rhiannon::Info(cfg) => {
                rh_info::handle_info(verbose_enabled, cfg);
            },
        }
    }
}

