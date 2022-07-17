use std::env;
use std::process;

use minigrep::{Config,run};  // K_22716 这里要用花括号
// use minigrep::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");

        process::exit(1);
    }
}
