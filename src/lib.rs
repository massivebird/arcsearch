use arcconfig::{read_config, System};
use regex::Regex;
use self::config::Config;
use std::{fs::DirEntry, result::Result, io};

pub mod config;

fn query_system(
    config: &Config,
    system: &System,
    num_matches: &mut u32,
) {
    let system_path = &(config.archive_root.clone() + "/" + system.directory.as_str());

    // saves a lot of indentation in the `for` loop
    let archive_iterator = || {
        std::path::Path::new(system_path)
            .read_dir()
            .unwrap()
            .into_iter()
            .filter_map(Result::ok)
            .filter(is_not_bios_dir)
    };

    for entry in archive_iterator() {
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
                .expect("unable to extract file name from entry")
                .to_string_lossy(),
        );

        if config.query.is_match(game_name) {
            println!("[ {} ] {game_name}", system.pretty_string);
            *num_matches += 1;
        }
    }
}

pub fn run(config: &Config) -> Result<(), io::Error> {
    let systems: Vec<System> = read_config(&config.archive_root)
        .into_iter()
        .filter(|s| config.desired_systems.clone().map_or(
            true,
            |labels| labels.contains(&s.label)
        ))
        .collect();


    let mut num_matches: u32 = 0;

    for system in systems {
        query_system(config, &system, &mut num_matches);
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

fn clean_game_name(game_name: &str) -> String {
    let patterns = [r"\(.*\)", r"\[.*\]", r"\.[^ ]+$"];
    Regex::new(&patterns.join("|"))
        .expect("invalid replace expression")
        .replace_all(game_name, "")
        .trim_end()
        .to_string()
}

fn is_not_bios_dir(entry: &DirEntry) -> bool {
    !entry.path().to_string_lossy().contains("!bios")
}

