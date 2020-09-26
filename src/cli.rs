use clap::{
    App,
    Arg,
    ArgMatches,
    crate_version,
    crate_authors,
    crate_name,
    crate_description,
};

pub fn cli() -> ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .about("Path to Cargo.toml file")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .about("Show up to date dependencies as well")
                .required(false)
        )
        .get_matches()
}
