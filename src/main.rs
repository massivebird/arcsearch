use self::config::Config;
use arcconfig::{read_config, system::System};
use regex::Regex;
use std::{collections::VecDeque, fs::DirEntry, path::Path, result::Result};
use tokio::spawn;

mod config;

#[tokio::main]
async fn main() {
    let config = Config::generate();

    let systems: Vec<System> = read_config(&config.archive_root)
        .into_iter()
        .filter(|s| {
            config
                .desired_systems
                .clone()
                .is_none_or(|labels| labels.contains(&s.label))
        })
        .collect();

    let mut handles = VecDeque::new();

    for system in systems.clone() {
        let config = config.clone();
        handles.push_back(spawn(async { query_system(config, system) }));
    }

    let mut num_matches: u32 = 0;

    for system in systems {
        let games = handles.pop_front().unwrap().await.unwrap();

        num_matches += u32::try_from(games.len()).unwrap();

        for game in games {
            println!("[ {} ] {game}", system.pretty_string);
        }
    }

    println!(
        "{num_matches} {noun} found.",
        noun = match num_matches {
            1 => "game",
            _ => "games",
        }
    );
}

fn query_system(config: Config, system: System) -> Vec<String> {
    let mut games: Vec<String> = Vec::new();

    let system_path = format!("{}/{}", config.archive_root, system.directory);

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
        let filename = {
            // Do not modify directory names since they do not have extensions.
            if entry.path().is_dir() {
                path.file_name().unwrap().to_string_lossy()
            } else {
                path.file_stem()
                    .expect("unable to extract file name from entry")
                    .to_string_lossy()
            }
        };

        // "Shadowrun"
        let game_name = clean_game_name(&filename).trim_end();

        if config.query.is_match(game_name) {
            games.push(game_name.to_string());
            // println!("[ {} ] {game_name}", system.pretty_string);
            // *num_matches += 1;
        }
    }

    games
}

fn clean_game_name(game_name: &str) -> &str {
    let patterns = [r"\(.*\)", r"\[.*\]"];

    Regex::new(&patterns.join("|"))
        .expect("invalid replace expression")
        .find(game_name)
        .map_or(game_name, |idx| &game_name[..idx.start()])
}

fn is_not_bios_dir(entry: &DirEntry) -> bool {
    !entry.path().to_string_lossy().contains("!bios")
}
