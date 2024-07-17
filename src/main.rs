use self::config::Config;
use arcconfig::{read_config, system::System};
use regex::Regex;
use std::path::Path;
use std::{fs::DirEntry, result::Result};

mod config;

fn main() {
    let config = Config::generate();

    let systems: Vec<System> = read_config(&config.archive_root)
        .into_iter()
        .filter(|s| {
            config
                .desired_systems
                .clone()
                .map_or(true, |labels| labels.contains(&s.label))
        })
        .collect();

    let mut num_matches: u32 = 0;

    for system in systems {
        query_system(&config, &system, &mut num_matches);
    }

    println!(
        "{num_matches} {noun} found.",
        noun = match num_matches {
            1 => "game",
            _ => "games",
        }
    );
}

fn query_system(config: &Config, system: &System, num_matches: &mut u32) {
    let system_path = format!(
        "{}/{}",
        config.archive_root.clone(),
        system.directory.as_str()
    );

    for entry in Path::new(&system_path)
        .read_dir()
        .unwrap()
        .filter_map(Result::ok)
        .filter(is_not_bios_dir)
    {
        if system.games_are_directories && entry.path().is_file()
            || !system.games_are_directories && entry.path().is_dir()
        {
            continue;
        }

        let path = &entry.path();
        let filename = path
            .file_name()
            .expect("unable to extract file name from entry")
            .to_string_lossy();

        // "Shadowrun"
        let game_name = &clean_game_name(&filename);

        if config.query.is_match(game_name) {
            println!("[ {} ] {game_name}", system.pretty_string);
            *num_matches += 1;
        }
    }
}

fn clean_game_name(game_name: &str) -> &str {
    let patterns = [r"\(.*\)", r"\[.*\]", r"\.[^ ]+$"];
    Regex::new(&patterns.join("|"))
        .expect("invalid replace expression")
        .find(game_name)
        .map_or(game_name, |i| &game_name[..i.start() - 1])
}

fn is_not_bios_dir(entry: &DirEntry) -> bool {
    !entry.path().to_string_lossy().contains("!bios")
}
