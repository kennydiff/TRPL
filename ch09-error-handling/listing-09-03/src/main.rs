use std::fs::File;

fn main() {
    let greeting_file_result: u32 = File::open("hello.txt");
}
