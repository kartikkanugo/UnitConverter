use std::{env, process};
use uconv::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        Config::help();
        process::exit(1);
    });

    config.run();

    println!("Hello, world!");
}
