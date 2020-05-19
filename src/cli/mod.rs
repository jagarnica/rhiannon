use std::env;
use structopt::StructOpt;

//* This processes the arguements passed from the cli
#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "The Fastest Streaming Discord Bot")]
pub struct Cli {
    #[structopt(short = "v", global = true, long = "verbose")]
    pub verbose: bool,
    #[structopt(subcommand)] 
    pub commands: Option<Subcommands>
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
