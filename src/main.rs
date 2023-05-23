use walkdir::WalkDir;
use regex::Regex;
use colored::*;
use std::env;

struct System {
    pretty_string: ColoredString,
    directory: String,
    games_are_directories: bool,
}

impl System {
    fn new(pretty_string: ColoredString, dir_name: &str, games_are_directories: bool) -> System {
        System {
            directory: String::from(dir_name),
            pretty_string,
            games_are_directories,
        }
    }
}

fn main() {
    let archive_root: String = String::from("/home/penguino/game-archive");

    let args: Vec<String> = env::args().collect();

    let query = match args.len() {
        1 => "", // match all
        2.. => &args[1],
        _ => panic!("std::env::args() smoked BAD weed"),
    };
    // convert string to case-insensitive regex query
    let query = Regex::new(&format!("(?i){query}")).unwrap();

    let systems = [
        System::new("3DS".truecolor(215,0,0), "3ds", false),
        System::new("DS".truecolor(135,215,255), "ds", false),
        System::new("GB".truecolor(95,135,95), "gb", false),
        System::new("GBA".truecolor(255,175,255), "gba", false),
        System::new("GCN".truecolor(135,95,255), "games", true),
        System::new("GEN".truecolor(88,88,88), "gen", false),
        System::new("N64".truecolor(0,215,135), "n64", false),
        System::new("NES".truecolor(215,0,0), "nes", false),
        System::new("PS1".truecolor(178,178,178), "nes", false),
        System::new("PS2".truecolor(102,102,102), "nes", false),
        System::new("PSP".truecolor(95,135,255), "nes", false),
        System::new("SNES".truecolor(95,0,255), "nes", false),
        System::new("WII".truecolor(0,215,255), "wbfs", true),
    ];

    let mut games_matched: u32 = 0;

    // silently skip error entries
    for entry in WalkDir::new(archive_root.clone()).into_iter().filter_map(|e| e.ok()) {
        // "snes/Shadowrun.sfc"
        let relative_pathname = entry.path().strip_prefix(&archive_root).unwrap()
            .to_str().unwrap().to_string();
        // "snes"
        let base_dir = relative_pathname[..relative_pathname.find("/").unwrap_or(0)].to_string();
        // "Shadowrun"
        let file_name = entry.path().file_stem().unwrap().to_str().unwrap();

        // dbg!(relative_pathname);

        let Some(system) = systems.iter()
            .filter(|s| s.directory == base_dir).next()
        else {
            continue;
        };

        if system.games_are_directories && entry.path().is_file() {
            continue;
        }
        
        if query.is_match(file_name) {
            println!("[ {} ] {}", system.pretty_string, file_name);
            games_matched += 1;
        }
    }

    println!("{games_matched} games found.");

}
