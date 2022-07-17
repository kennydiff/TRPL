use std::env;
use std::process;

use minigrep::Config;

// ANCHOR: here
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");  // K_22716 使用 eprintln! 将错误信息写入标准错误而不是标准输出
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");

        process::exit(1);
    }
}
// ANCHOR_END: here
