use archive_systems::generate_systems;
use regex::Regex;
use std::result::Result;
use std::{env, io, process};
use walkdir::{DirEntry, WalkDir};

pub struct Config {
    archive_root: String,
    query: Regex,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let query = args.get(1).unwrap_or(&String::new()).clone();
        let query = Regex::new(&format!("(?i){query}"))
            .expect("invalid regular expression argument");
        let archive_root = args
            .get(2)
            .unwrap_or(&env::var("VG_ARCHIVE").unwrap_or_else(|_| {
                eprintln!("Neither provided path nor VG_ARCHIVE are valid");
                process::exit(1);
            }))
            .clone();

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
    let systems = generate_systems();

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
