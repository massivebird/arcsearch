use std::io;
use arcsearch::Config;

fn main() -> io::Result<()> {
    let config = Config::new();

    arcsearch::run(&config)?;

    Ok(())
}
