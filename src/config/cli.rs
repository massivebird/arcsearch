use clap::{Arg, ValueHint, Command, command};

pub fn build_args() -> Command {
    command!()
        .arg(Arg::new("desired_systems")
            .long("systems")
            .help("Comma-separated system labels to query exclusively")
            .value_name("labels")
        )
        .arg(Arg::new("archive_root")
            .long("archive-root")
            .alias("archive-path")
            .help("The root of your game archive")
            .value_name("PATH")
            .value_hint(ValueHint::DirPath)
        )
        .arg(Arg::new("query")
            .required(true)
            .help("Regex search query")
        )
}
