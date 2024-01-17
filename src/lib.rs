use arcconfig::{read_config, System};
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

fn query_system(
    config: &Config,
    system: System,
) -> u32 {
    let mut num_matches: u32 = 0;

    let is_not_bios_dir = |entry: &DirEntry| !entry.path().to_string_lossy().contains("!bios");

    // saves a lot of indentation in the `for` loop
    let walk_through_archive = || {
        WalkDir::new(config.archive_root.clone() + "/" + system.directory.as_str())
            .max_depth(1)
            .into_iter()
            // silently skip errorful entries
            .filter_map(Result::ok)
            .filter(is_not_bios_dir)
            // skip directory itself
            .skip(1)
    };

    for entry in walk_through_archive() {
        // dbg!(&entry);
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

    num_matches
}

pub fn run(config: &Config) -> Result<(), io::Error> {
    let systems = read_config(&config.archive_root);

    let mut num_matches: u32 = 0;

    for system in systems {
        num_matches += query_system(config, system);
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
