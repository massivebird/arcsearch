use colored::{Colorize, ColoredString};

pub struct System {
    pub pretty_string: ColoredString,
    pub directory: String,
    pub games_are_directories: bool,
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

pub fn generate_systems() -> [System; 13] {
    [
        System::new("3DS".truecolor(215,0,0), "3ds", false),
        System::new("DS".truecolor(135,215,255), "ds", false),
        System::new("GB".truecolor(95,135,95), "gb", false),
        System::new("GBA".truecolor(255,175,255), "gba", false),
        System::new("GCN".truecolor(135,95,255), "games", true),
        System::new("GEN".truecolor(88,88,88), "gen", false),
        System::new("N64".truecolor(0,215,135), "n64", false),
        System::new("NES".truecolor(215,0,0), "nes", false),
        System::new("PS1".truecolor(178,178,178), "ps1", true),
        System::new("PS2".truecolor(102,102,102), "ps2", false),
        System::new("PSP".truecolor(95,135,255), "psp", false),
        System::new("SNES".truecolor(95,0,255), "snes", false),
        System::new("WII".truecolor(0,215,255), "wbfs", true),
    ]
}
