use std::env;
use structopt::StructOpt;

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
