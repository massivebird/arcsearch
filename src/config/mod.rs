use arcconfig::{config_file::ConfigFile, system::System};
use regex::Regex;
use std::{env, path::PathBuf};

mod cli;

#[derive(Clone)]
pub struct App {
    pub query: Regex,
    pub archive_root: PathBuf,

    /// Systems as specified by configuration file.
    pub systems: Vec<System>,

    /// Print game titles as their plain filenames.
    pub titles_as_filenames: bool,
    /// Only print number of matching games.
    pub count_mode: bool,
}

impl App {
    /// Generates configuration options based on command line arguments.
    pub fn build() -> Self {
        let cli = cli::build();

        let matches = cli.get_matches();

        // Generate CLI completions for specified shell, then exit.
        if let Some(sub_matches) = matches.subcommand_matches("completions") {
            let shell = sub_matches
                .get_one::<clap_complete_command::Shell>("shell")
                .unwrap();

            let mut cli = cli::build();

            shell.generate(&mut cli, &mut std::io::stdout());

            std::process::exit(0);
        }

        // Shortcut for retrieving a command line argument.
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

            let opts = if matches.get_flag("case_sensitive") {
                ""
            } else {
                "(?i)"
            };

            Regex::new(&format!("{opts}{raw_query}")).expect("Invalid regex query")
        };

        let systems: Vec<System> = {
            // If the user specified some system labels, only query these systems.
            let desired_systems: Option<Vec<String>> = get_arg("desired_systems")
                .map(|labels| labels.split(',').map(ToString::to_string).collect());

            ConfigFile::from_archive(&archive_root)
                .unwrap()
                .systems()
                .unwrap()
                .into_iter()
                .filter(|s| {
                    desired_systems
                        .as_ref()
                        .is_none_or(|labels| labels.contains(&s.label))
                })
                .collect()
        };

        Self {
            systems,
            archive_root,
            query,
            titles_as_filenames: matches.get_flag("filenames"),
            count_mode: matches.get_flag("count"),
        }
    }
}
