use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);  // K_22716 dbg -- Debug
}
