use std::{env, io};
use arcsearch::Config;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    arcsearch::run(&config)?;

    Ok(())
}
