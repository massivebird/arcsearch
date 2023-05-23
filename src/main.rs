use walkdir::WalkDir;
use regex::Regex;
use colored::*;
use std::fs;

struct System {
    pretty_string: ColoredString,
    directory: String,
    games_as: GamesAs,
}

enum GamesAs {
    Files,
    Directories,
}

impl System {
    fn new(pretty_string: ColoredString, dir_name: &str, games_as: GamesAs) -> System {
        System {
            directory: String::from(dir_name),
            pretty_string,
            games_as,
        }
    }
}

fn main() {
    let archive_root: String = String::from("/home/penguino/game-archive");
    let query: &str = "";
    // let query = Regex::new(r"{}").unwrap();
    let query = Regex::new(query).unwrap();

    let systems = [
        System::new("GBA".purple(), "gba", GamesAs::Files),
        System::new("GB".bright_green(), "gb", GamesAs::Files),
        System::new("DS".bright_blue(), "ds", GamesAs::Files),
        System::new("N64".green(), "n64", GamesAs::Files),
        System::new("GCN".bright_magenta(), "games", GamesAs::Directories),
        System::new("WII".blue(), "wbfs", GamesAs::Directories),
    ];

    // silently skip error entries
    for entry in WalkDir::new(archive_root.clone()).into_iter().filter_map(|e| e.ok()) {
        let relative_pathname = entry.path().strip_prefix(&archive_root).unwrap()
            .to_str().unwrap().to_string();
        let file_name = entry.path().file_stem().unwrap().to_str().unwrap();
        let base_dir = relative_pathname[..relative_pathname.find("/").unwrap_or(0)].to_string();

        for valid_system_dir in systems.iter().map(|s| s.directory.clone()) {
            if base_dir == valid_system_dir {

            }
        }

        // dbg!(relative_pathname);

        let Some(system) = systems.iter()
            .filter(|s| s.directory == base_dir).next()
        else {
            continue;
        };
        
        if query.is_match(file_name) {
            println!("{} {}", system.pretty_string, file_name);
        }
    }

}
