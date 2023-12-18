use arcconfig::read_config;
use clap::Arg;
use regex::Regex;
use std::{result::Result, env, io};
use walkdir::{DirEntry, WalkDir};

pub struct Config {
    archive_root: String,
    query: Regex,
}

impl Config {
    pub fn new() -> Self {
        let matches = clap::command!()
            .arg(Arg::new("archive_root")
                .long("archive-root")
                .alias("archive-path")
                .help("The root of your game archive")
            )
            .arg(Arg::new("query")
                .default_value(".*")
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

        Self {
            archive_root,
            query,
        }
    }
}

fn clean_game_name(game_name: &str) -> String {
    let patterns = [r"\(.*\)", r"\[.*\]", r"\.[^ ]+$"];
    Regex::new(&patterns.join("|"))
        .expect("invalid replace expression")
        .replace_all(game_name, "")
        .trim_end()
        .to_string()
}

pub fn run(config: &Config) -> Result<(), io::Error> {
    let systems = read_config(&config.archive_root);

    let is_valid_system_dir = |entry: &DirEntry| {
        systems
            .iter()
            .any(|system| entry.path().to_string_lossy().contains(&system.directory))
    };

    let is_not_bios_dir = |entry: &DirEntry| !entry.path().to_string_lossy().contains("!bios");

    let mut num_matches: u32 = 0;

    for entry in WalkDir::new(&config.archive_root)
        .into_iter()
        // silently skip errorful entries
        .filter_map(Result::ok)
        .filter(|e| is_not_bios_dir(e) && is_valid_system_dir(e))
        {
            // "snes/Shadowrun.sfc"
            let relative_pathname = entry
                .path()
                .strip_prefix(&config.archive_root)
                .expect("path does not contain archive root")
                .to_string_lossy();

            // "snes"
            let base_dir = relative_pathname[..relative_pathname.find('/').unwrap_or(0)].to_string();

            let Some(system) = systems.iter()
                .find(|system| system.directory == base_dir)
            else {
                continue;
            };

            if system.games_are_directories && entry.path().is_file() {
                continue;
            }

            // "Shadowrun"
            let game_name = &clean_game_name(
                &entry
                    .path()
                    .file_name()
                    .expect("unable to extract file name from path")
                    .to_string_lossy(),
            );

            if config.query.is_match(game_name) {
                println!("[ {} ] {}", system.pretty_string, game_name);
                num_matches += 1;
            }
        }

    println!(
        "{num_matches} {noun} found.",
        noun = match num_matches {
            1 => "game",
            _ => "games",
        }
    );

    Ok(())
}
