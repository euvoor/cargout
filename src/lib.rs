use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::env::current_dir;

use reqwest::header::USER_AGENT;
use reqwest::Client;
use toml::Value as TomlValue;
use tracing::{ error, trace };
use clap::ArgMatches;
use serde_json::Value as JsonValue;

pub async fn get_new_version(dep: String) -> io::Result<String> {
    let crate_url = format!("https://crates.io/api/v1/crates/{}", dep);

    trace!("[ FETCH STARTED ] {}", crate_url);

    let data = Client::builder()
        .build().unwrap()
        .get(&crate_url)
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/83.0.4103.116 Safari/537.36")
        .send().await.unwrap()
        .text().await.unwrap();

    let data: JsonValue = serde_json::from_str(&data)?;

    trace!("[ FETCH END ] {}", crate_url);

    Ok(data["versions"][0]["num"].as_str().unwrap().to_string())
}

pub fn get_deps(matches: &ArgMatches) -> io::Result<TomlValue> {
    let mut path_buf;

    if let Some(file) = matches.value_of("file") {
        path_buf = PathBuf::from(file);
    } else {
        path_buf = current_dir().unwrap();
    }

    path_buf.push("Cargo.toml");

    if path_buf.exists() {
        let mut file = File::open(path_buf)?;
        let mut buf = String::new();

        trace!("[ CARGO PARSER ] reading Cargo.toml file");

        file.read_to_string(&mut buf)?;
        let value = buf.parse::<TomlValue>()?;

        return Ok(value["dependencies"].clone())
    }

    error!("{:?} does not exists!", path_buf);
    panic!();
}
