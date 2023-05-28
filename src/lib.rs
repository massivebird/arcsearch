use walkdir::{WalkDir, DirEntry};
use regex::Regex;
use std::env;
use archive_systems::generate_systems;

pub struct Config {
    archive_root: String,
    query: Regex,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        let query = args.get(1).unwrap_or(&"".to_string()).to_owned();
        let query = Regex::new(&format!("(?i){query}")).unwrap();
        let archive_root = args.get(2)
            .unwrap_or(&env::var("VG_ARCHIVE")
                .unwrap_or_else(
                    |_| panic!("Neither provided path nor VG_ARCHIVE are valid")
                )
            )
            .to_owned();

        Config { archive_root, query }
    }
}

fn clean_game_name(game_name: String) -> String {
    let patterns = [
        r"\(.*\)",
        r"\[.*\]",
        r"\.[^ ]+$",
    ];
    Regex::new(&patterns.join("|")).unwrap()
        .replace_all(&game_name, "").trim_end().to_string()
}

pub fn run(config: Config) {
    let systems = generate_systems();

    let is_valid_system_dir = |e: &DirEntry| {
        systems.iter().any(|s| e.path().to_string_lossy().contains(&s.directory))
    };

    let is_not_bios_dir = |e: &DirEntry| {
        !e.path().to_string_lossy().contains("!bios")
    };

    let mut num_matches: u32 = 0;

    for entry in WalkDir::new(&config.archive_root).into_iter()
        .filter_map(|e| e.ok()) // silently skip errorful entries
        .filter(|e| is_not_bios_dir(e) && is_valid_system_dir(e))
        {
            // "snes/Shadowrun.sfc"
            let relative_pathname = entry.path()
                .strip_prefix(&config.archive_root).unwrap()
                .to_string_lossy();

            // "snes"
            let base_dir = relative_pathname
                [..relative_pathname.find("/").unwrap_or(0)]
                .to_string();

            // "Shadowrun"
            let game_name = &clean_game_name(entry.path().file_name()
                .unwrap().to_string_lossy().into_owned()
            );

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
