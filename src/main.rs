use self::config::App;
use arcconfig::system::System;
use regex::Regex;
use std::{collections::VecDeque, fs::DirEntry, path::Path, result::Result};
use tokio::spawn;

mod config;

#[tokio::main]
async fn main() {
    let app = App::generate();

    let mut handles = VecDeque::new();

    for system in app.systems.clone() {
        let config = app.clone();
        handles.push_back(spawn(async move { query_system(&config, system) }));
    }

    let mut num_matches: u32 = 0;

    for system in app.systems {
        let games = handles.pop_front().unwrap().await.unwrap();

        num_matches += u32::try_from(games.len()).unwrap();

        // Check if user suppresses matches output.
        if app.count_mode {
            continue;
        }

        for game in games {
            println!("[ {system} ] {game}");
        }
    }

    if app.count_mode {
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

fn query_system(config: &App, system: System) -> Vec<String> {
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
