use arcsearch::config::Config;
use std::io;

fn main() -> io::Result<()> {
    let config = Config::new();

    arcsearch::run(&config)?;

    Ok(())
}
