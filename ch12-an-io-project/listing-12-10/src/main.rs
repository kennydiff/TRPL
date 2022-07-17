use std::env;
use std::fs;
// ANCHOR: here
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {  // K_22716 这里的|err|{...}是个<闭包> 也叫匿名函数
        println!("Problem parsing arguments: {err}");  // K_22716 err是这个匿名函数的参数
        process::exit(1);
    });

    // --snip--
    // ANCHOR_END: here

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
