use regex::Regex;
use std::{env, path::PathBuf};

mod cli;

#[derive(Clone)]
pub struct Config {
    pub archive_root: PathBuf,
    pub query: Regex,
    pub desired_systems: Option<Vec<String>>,
}

impl Config {
    /// Generates configuration options based on command line arguments.
    pub fn generate() -> Self {
        let matches = cli::build_args().get_matches();

        let get_arg = |arg_name: &str| -> Option<&String> { matches.get_one::<String>(arg_name) };

        let archive_root: PathBuf = {
            let value = get_arg("archive_root").map_or_else(
            || env::var("VG_ARCHIVE").unwrap_or_else(
                |_| panic!("Please supply an archive path via argument or VG_ARCHIVE environment variable.")
            ),
            String::to_string
        );

            PathBuf::from(value)
        };

        let query: Regex = {
            let raw_query = if matches.get_flag("all") {
                "."
            } else {
                get_arg("query").unwrap()
            };

            Regex::new(&format!("(?i){raw_query}")).expect("Invalid regex query")
        };

        let desired_systems: Option<Vec<String>> = get_arg("desired_systems")
            .map(|labels| labels.split(',').map(ToString::to_string).collect());

        Self {
            archive_root,
            query,
            desired_systems,
        }
    }
}
