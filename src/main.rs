mod api;
mod config;

use clap::Parser;
use std::fs;
use tokio;

use api::Flash;
use config::Config;

#[derive(Debug, Parser)]
struct Args {
    #[clap(
        short = 'c',
        long = "config",
        value_name = "filename",
        default_value = "upmkplay.config.json"
    )]
    /// Specify config filename
    config: String,

    /// AiScript filename
    script: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config::load(&args.config);
    let flash = Flash::new(config);
    let script = args
        .script
        .map(|filename| fs::read_to_string(&filename).unwrap());
    flash.update(script).await;
    println!("Updated the Play!");
}
