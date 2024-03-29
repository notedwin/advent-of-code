use std::env;
use std::process;

use minigrep::Config;

fn main(){
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    eprintln!("Searching for {}", config.query);
    eprintln!("In file \x1b[0;31m{}\x1b[0m", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
