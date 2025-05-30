use clap::ValueEnum;
use clap_complete::{generate_to, Shell};
use std::env;
use std::io::Error;

include!("src/config/cli.rs");

fn main() -> Result<(), Error> {
    let Some(out_dir) = env::var_os("OUT_DIR") else {
        return Ok(());
    };

    let bin_name = "arcsearch";

    let mut cmd = build_args();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, bin_name, &out_dir)?;
    }

    Ok(())
}

