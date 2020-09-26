use std::io;

use tracing::trace;
use console::style;
use futures::future::join_all;

pub mod cli;
pub use cli::cli;

use cargout::get_deps;
use cargout::get_new_version;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt()
        //.with_env_filter("cargout=trace")
        .init();

    trace!("[ STARTED ]");

    let matches = cli();

    if let Ok(deps) = get_deps(matches) {
        let mut tasks = vec![];

        for (dep, value) in deps.as_table().unwrap() {
            let dep = dep.to_string();
            let mut version = String::new();

            if value.is_table() {
                version = value.as_table().unwrap()["version"].as_str().unwrap().to_string();
            } else {
                version = value.as_str().unwrap().to_string();
            }

            let task = tokio::spawn(async move {
                let new_version = get_new_version(dep.clone()).await.unwrap();

                if version == new_version {
                    println!("{:20} {:10} {:10}", dep, version, new_version);
                } else {
                    println!(
                        "{:20} {:10} {:10}",
                        style(dep).red(),
                        style(version).red(), style(new_version).red()
                    );
                }
            });

            tasks.push(task);
        }

        join_all(tasks).await;
    }

    trace!("[ FINISHED ]");

    Ok(())
}
