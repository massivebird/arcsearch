use arcconfig::read_config;
use regex::Regex;
use self::config::Config;
use std::string::ToString;
use std::{result::Result, io};
use walkdir::{DirEntry, WalkDir};

pub mod config;

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

    // saves a lot of indentation in the `for` loop
    let walk_through_archive = || {
        WalkDir::new(&config.archive_root)
            // 1) iterates through regular files before subdirectories, and
            // 2) iterates through both types lexicographically
            .sort_by(|a, b| a.file_name().cmp(b.file_name()))
            .into_iter()
            // silently skip errorful entries
            .filter_map(Result::ok)
            .filter(|e| is_not_bios_dir(e) && is_valid_system_dir(e))
    };

    for entry in walk_through_archive() {
        // "snes/Shadowrun.sfc"
        let relative_pathname = entry
            .path()
            .strip_prefix(&config.archive_root)
            .expect("path does not contain archive root")
            .to_string_lossy();

        // "snes"
        let base_dir = relative_pathname[..relative_pathname.rfind('/').unwrap_or(0)].to_string();

        let Some(system) = systems.iter().find(
            |system| system.directory == base_dir
        ) else {
            continue;
        };

        if config.desired_systems.is_some()
        && !config.desired_systems.as_ref().unwrap().contains(&system.label) {
            continue;
        }

        if system.games_are_directories && entry.path().is_file() ||
        !system.games_are_directories && entry.path().is_dir()
        {
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
