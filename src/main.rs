use alarm::Config;
use std::env;
use std::process;

fn main() {
    let time = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    time.countdown();
}
