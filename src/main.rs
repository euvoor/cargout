use std::io;

use tracing::{ trace, error };
use console::style;
use futures::future::join_all;
use tokio::sync::mpsc::unbounded_channel;
use tracing_subscriber::{ fmt, EnvFilter };
use tracing_subscriber::prelude::*;

pub mod cli;
pub use cli::cli;

use cargout::get_deps;
use cargout::get_new_version;

#[tokio::main]
async fn main() -> io::Result<()> {
    let fmt_layer = fmt::layer().with_target(false);
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("error"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    trace!("[ STARTED ]");

    let matches = cli();

    if let Ok(deps) = get_deps(&matches) {
        let mut tasks = vec![];
        let (tx, mut rx) = unbounded_channel();
        let (out_tx, mut out_rx) = unbounded_channel();

        tokio::spawn(async move {
            while let Some(line) = out_rx.recv().await {
                if line == "/end" {
                    break
                }

                println!("{}", line);
            }
        });

        let arg_all = matches.is_present("all");

        tokio::spawn(async move {
            let mut lines = vec![];

            while let Some(line) = rx.recv().await {
                if line == "/end" {
                    break
                } else {
                    lines.push(line);
                }
            }

            if arg_all {
                for line in lines {
                    println!("{}", line);
                }
            }
        });

        for (dep, value) in deps.as_table().unwrap() {
            let dep = dep.to_string();
            let mut version = String::new();

            if value.is_table() {
                version = value.as_table().unwrap()["version"].as_str().unwrap().to_string();
            } else {
                version = value.as_str().unwrap().to_string();
            }

            let tx = tx.clone();
            let out_tx = out_tx.clone();

            let task = tokio::spawn(async move {
                let new_version = get_new_version(dep.clone()).await.unwrap();

                if version == new_version {
                    tx.send(format!("{:20} {:10} {:10}", dep, version, new_version)).unwrap();
                } else {
                    out_tx.send(format!("{:20} {:10} {:10}", style(dep).red(), style(version).red(), style(new_version).red())).unwrap();
                }
            });

            tasks.push(task);
        }

        join_all(tasks).await;

        tx.send(String::from("/end")).unwrap();
    }

    trace!("[ FINISHED ]");

    Ok(())
}
