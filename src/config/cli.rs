use clap::{command, Arg, ArgGroup, Command, ValueHint};

pub fn build_cli() -> Command {
    let query_long_help = "\
        Regex pattern used to query the archive.
";

    let desired_systems_long_help = "\
        Query specified systems with comma-separated system labels.\
";

    let archive_root_long_help = "\
        Provide the path to your archive root, overriding the VG_ARCHIVE environment variable if it exists.\
";

    let all_long_help = "\
        Display all games. Incompatible with the <query> argument.\
";

    let filenames_long_help = "\
        Print filenames as they appear in the file system, rather than truncate certain elements. Retains file extensions, regional codes (e.g. \"(USA, Europe)\"), disk indicators (e.g. \"(Disk 2)\"), and other codes.\
";

    command!()
        .args_conflicts_with_subcommands(true)
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg(
                    Arg::new("shell")
                        .required(true)
                        .value_name("shell")
                        .value_parser(
                            clap::builder::EnumValueParser::<clap_complete_command::Shell>::new(),
                        ),
                ),
        )
        .arg(
            Arg::new("query")
                .required(true)
                .hide(true)
                .help("Regular expression query")
                .long_help(query_long_help),
        )
        .next_help_heading("Search options")
        .args([
            Arg::new("archive_root")
                .short('r')
                .long("archive-root")
                .alias("archive-path")
                .help("Provide the path to your archive root")
                .long_help(archive_root_long_help)
                .value_name("path")
                .value_hint(ValueHint::DirPath),
            Arg::new("all")
                .short('a')
                .long("all")
                .required(false)
                .conflicts_with("query")
                .action(clap::ArgAction::SetTrue)
                .help("Display all games")
                .long_help(all_long_help),
            Arg::new("desired_systems")
                .short('s')
                .long("systems")
                .help("Query specified systems")
                .long_help(desired_systems_long_help)
                .value_name("labels"),
            Arg::new("case_sensitive")
                .long("case-sensitive")
                .action(clap::ArgAction::SetTrue)
                .help("Execute query case sensitively"),
        ])
        .next_help_heading("Output options")
        .args([
            Arg::new("count")
                .short('c')
                .long("count")
                .action(clap::ArgAction::SetTrue)
                .help("Only print number of matches"),
            Arg::new("filenames")
                .short('f')
                .long("filenames")
                .action(clap::ArgAction::SetTrue)
                .help("Print game titles as raw filenames")
                .long_help(filenames_long_help),
        ])
}
