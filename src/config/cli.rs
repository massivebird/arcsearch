use clap::{command, Arg, Command, ValueHint};

pub fn build_args() -> Command {
    let query_long_help = "\
        Regex pattern used to query the archive.
";

    let desired_systems_long_help = "\
        Query specified systems with comma-seperated system labels.\
";

    let archive_root_long_help = "\
        Provide the path to your archive root, overriding the VG_ARCHIVE environment variable if it exists.\
";

    let all_long_help = "\
        Display all games. Incompatible with the <query> argument.\
";

    command!().args([
        Arg::new("desired_systems")
            .short('s')
            .long("systems")
            .help("Query specified systems")
            .long_help(desired_systems_long_help)
            .value_name("labels"),
        Arg::new("archive_root")
            .short('r')
            .long("archive-root")
            .alias("archive-path")
            .help("Provide the path to your archive root")
            .long_help(archive_root_long_help)
            .value_name("PATH")
            .value_hint(ValueHint::DirPath),
        Arg::new("all")
            .short('a')
            .long("all")
            .required(false)
            .conflicts_with("query")
            .action(clap::ArgAction::SetTrue)
            .help("Display all games")
            .long_help(all_long_help),
        Arg::new("query")
            .required(true)
            .help("Regex query")
            .long_help(query_long_help),
    ])
}
