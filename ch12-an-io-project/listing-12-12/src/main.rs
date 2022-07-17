use std::env;
use std::fs;
use std::process;
// ANCHOR: here
use std::error::Error;

// --snip--

// ANCHOR_END: here

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

// ANCHOR: here
fn run(config: Config) -> Result<(), Box<dyn Error>> {  // K_22716 dyn，它是 “动态的”（“dynamic”）的缩写
    let contents = fs::read_to_string(config.file_path)?;  // K_22716 "?" 会从函数中返回错误值并让调用者来处理它

    println!("With text:\n{contents}");

    Ok(())  // K_22716 unit 类型 "()" , 一个空的tuple， 将 unit 类型值包装进 Ok 值中
}
// ANCHOR_END: here

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
