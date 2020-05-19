use structopt::StructOpt;
use std::env;

//* This processes the arguements passed from the cli 
#[derive(StructOpt, Debug)]
#[structopt(about = "The Fastest Streaming Discord Bot")]
pub enum Cli {
    Start {
        /// Port number to run the bot from 
        #[structopt(short="v", long="verbose")]
        verbose: bool,
        #[structopt(flatten)]
        opts: StartOpts
    },
    Info { 
        #[structopt(flatten)]
        opts: InfoOpts
    },
}
#[derive(StructOpt, Debug)]
pub struct InfoOpts { 
    #[structopt(short="v", long="verbose")]
    verbose: bool,
    #[structopt(short="i", long="inputs")]
    inputs: bool,
    #[structopt(short="o", long="outputs")]
    outputs: bool,
    #[structopt(short="d", long="defaults")]
    defaults: bool,
    #[structopt(short="a", long="all")]
    all:bool
}

#[derive(StructOpt, Debug)]
pub struct StartOpts{
       /// Port number to run the bot from 
       #[structopt(short, long, default_value = "8008")]
       port: String,
}


