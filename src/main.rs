use std::env;
use arcsearch::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    arcsearch::run(config);
}
