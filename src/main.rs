use self::config::Config;
use arcconfig::{config_file::ConfigFile, system::System};
use regex::Regex;
use std::{collections::VecDeque, fs::DirEntry, path::Path, result::Result};
use tokio::spawn;

mod config;

#[tokio::main]
async fn main() {
    let config = Config::generate();

    let systems: Vec<System> = ConfigFile::from_archive(&config.archive_root)
        .unwrap()
        .systems()
        .unwrap()
        .into_iter()
        .filter(|s| {
            config
                .desired_systems
                .as_ref()
                .is_none_or(|labels| labels.contains(&s.label))
        })
        .collect();

    let mut handles = VecDeque::new();

    for system in systems.clone() {
        let config = config.clone();
        handles.push_back(spawn(async move { query_system(&config, system) }));
    }

    let mut num_matches: u32 = 0;

    for system in systems {
        let games = handles.pop_front().unwrap().await.unwrap();

        num_matches += u32::try_from(games.len()).unwrap();

        // Check if user suppresses matches output.
        if config.only_print_count {
            continue;
        }

        for game in games {
            println!("[ {system} ] {game}");
        }
    }

    if config.only_print_count {
        println!("{num_matches}");
    } else {
        println!(
            "{num_matches} {noun} found.",
            noun = match num_matches {
                1 => "game",
                _ => "games",
            }
        );
    }
}

fn query_system(config: &Config, system: System) -> Vec<String> {
    let mut games: Vec<String> = Vec::new();

    let system_path = config.archive_root.join(system.directory);

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
            let part = if entry.path().is_dir() || config.titles_as_filenames {
                path.file_name()
            } else {
                // Trim extension off of normal files, unless otherwise specified
                path.file_stem()
            };

            let Some(part) = part else {
                panic!("Failed to extract filename from path: {path:?}");
            };

            part.to_string_lossy()
        };

        // Format game name to user specification.
        // Could be "Pokemon Snap", or "Pokemon Snap (USA).n64"
        let game_name = {
            if config.titles_as_filenames {
                &filename
            } else {
                clean_game_name(&filename).trim_end()
            }
        };

        if config.query.is_match(game_name) {
            games.push(game_name.to_string());
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
