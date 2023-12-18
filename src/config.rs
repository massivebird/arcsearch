use clap::{Arg, ValueHint};
use regex::Regex;
use std::env;

pub struct Config {
    pub archive_root: String,
    pub query: Regex,
    pub desired_systems: Option<Vec<String>>,
}

impl Config {
    pub fn new() -> Self {
        let matches = clap::command!()
            .arg(Arg::new("desired_systems")
                .long("systems")
                .help("Comma-separated system labels to query exclusively")
                .value_name("labels")
            )
            .arg(Arg::new("archive_root")
                .long("archive-root")
                .alias("archive-path")
                .help("The root of your game archive")
                .value_name("PATH")
                .value_hint(ValueHint::DirPath)
            )
            .arg(Arg::new("query")
                .required(true)
                .help("Regex search query")
            )
            .get_matches();

        let archive_root: String = matches.get_one::<String>("archive_root").map_or_else(
            || env::var("VG_ARCHIVE").unwrap_or_else(
                |_| panic!("Please supply an archive path via argument or VG_ARCHIVE environment variable.")
            ),
            String::to_string
        );

        let query: Regex = {
            let raw_query = matches.get_one::<String>("query").unwrap();
            Regex::new(&format!("(?i){raw_query}")).expect("Invalid regex query")
        };

        let desired_systems: Option<Vec<String>> = matches.get_one::<String>("desired_systems").map(
            |labels| labels.split(',').map(ToString::to_string).collect()
        );

        dbg!(&desired_systems);

        Self {
            archive_root,
            query,
            desired_systems,
        }
    }
}
