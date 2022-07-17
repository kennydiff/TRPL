// ANCHOR: here
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    // ANCHOR_END: here
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // ANCHOR: here
    if let Err(e) = minigrep::run(config) {  // K_22716 这里注意 if let 的写法，替代了 match, Catch的是Err的错误
        // --snip--  // K_22716 执行 minigrep::run()函数,如果错了,则执行后续block里的代码
        // ANCHOR_END: here
        println!("Application error: {e}");

        process::exit(1);
        // ANCHOR: here
    }
}
// ANCHOR_END: here
