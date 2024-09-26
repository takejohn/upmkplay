mod api;
mod config;

use clap::Parser;
use notify::{RecursiveMode, Watcher};
use std::{path::Path, sync::mpsc};
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

    #[clap(short = 'w', long = "watch")]
    /// Enable watch mode
    watch: bool,

    #[clap(value_name = "script")]
    /// AiScript filename
    script: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = Config::load(&args.config);
    let flash = Flash::new(config);

    if let Some(filename) = args.script {
        flash.update_from_file(&filename).await;

        if args.watch {
            let (tx, rx) = mpsc::channel();
            let mut watcher = notify::recommended_watcher(tx).unwrap();
            watcher
                .watch(Path::new(&filename), RecursiveMode::NonRecursive)
                .unwrap();
            println!("Watching '{}'...", filename);

            loop {
                rx.recv().unwrap().unwrap();
                println!("'{}' Updated", filename);
                flash.update_from_file(&filename).await;
            }
        }
    } else {
        flash.update(None).await;
    }
}
