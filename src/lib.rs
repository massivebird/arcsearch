use walkdir::WalkDir;
use regex::Regex;
use colored::{Colorize, ColoredString};

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

pub struct Config {
    archive_root: String,
    query: Regex,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query = args.get(1).unwrap_or(&"".to_string()).to_owned();
        let query = Regex::new(&format!("(?i){query}")).unwrap();
        let archive_root = args.get(2).unwrap_or(&"/home/penguino/game-archive".to_string()).to_owned();

        Config { archive_root, query }
    }

}

fn clean_game_name(game_name: String) -> String {
    let patterns = [
        r"\(.*\)",
        r"\[.*\]"
    ];
    Regex::new(&patterns.join("|")).unwrap()
        .replace_all(&game_name, "").trim_end().to_string()
}

pub fn run(config: Config) {
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

    let mut num_matches: u32 = 0;

    // silently skip error entries
    for entry in WalkDir::new(&config.archive_root)
        .into_iter().filter_map(|e| e.ok())
        {

            // "snes/Shadowrun.sfc"
            let relative_pathname = entry.path()
                .strip_prefix(&config.archive_root).unwrap()
                .to_str().unwrap().to_string();
            // "snes"
            let base_dir = relative_pathname
                [..relative_pathname.find("/").unwrap_or(0)]
                .to_string();
            // "Shadowrun"
            let game_name = entry.path().file_stem().unwrap()
                .to_str().unwrap();
            let game_name = &clean_game_name(game_name.to_string());

            let Some(system) = systems.iter()
                .filter(|s| s.directory == base_dir).next()
            else {
                continue;
            };

            if system.games_are_directories && entry.path().is_file() {
                continue;
            }

            if config.query.is_match(game_name) {
                println!("[ {} ] {}", system.pretty_string, game_name);
                num_matches += 1;
            }
        }

    println!("{num_matches} {noun} found.",
        noun = match num_matches {
            1 => "game",
            _ => "games",
        });
}
