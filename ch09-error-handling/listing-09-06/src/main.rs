// ANCHOR: here
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(f_ile) => f_ile,
        Err(e2) => return Err(e2),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e3) => Err(e3),
    }
}
// ANCHOR_END: here

fn main() {
    let username_B = read_username_from_file().expect("Unable to get username");
    println!("Username is: {}", username_B);
}
