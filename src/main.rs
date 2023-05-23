use walkdir::WalkDir;
use regex::Regex;
use colored::*;


struct System {
    pretty_string: ColoredString,
    dir_name: String
}

impl System {
    fn new(in_pretty_string: ColoredString, in_dir_name: &str) -> Self {
        Self {
            dir_name: String::from(in_dir_name),
            pretty_string: in_pretty_string
        }
    }
}

fn main() {
    let archive_dir: String = String::from("/home/penguino/wiiback/");
    let query: &str = "Fake";

    let systems = [
        System::new("GBA".purple(), "gba"),
        System::new("GB".bright_green(), "gb"),
        System::new("N64".green(), "n64"),
        System::new("GCN".bright_magenta(), "games"),
    ];

    let match_query = Regex::new(query).unwrap();
    let match_extension = Regex::new(r"\..*").unwrap();
    let match_system = Regex::new(format!(r"{}/?\w*/", archive_dir).as_str()).unwrap();
    for entry in WalkDir::new(archive_dir) {
        let filepath = entry.unwrap();
        if filepath.file_type().is_dir() { continue }
        let game_name = filepath.file_name().to_os_string().into_string().unwrap();
        if match_query.find(game_name.as_str()).is_some() {
            println!("{} {}", filepath.path().to_str().unwrap(), match_extension.replace(game_name.as_str(), ""));
        }   
    }

}
