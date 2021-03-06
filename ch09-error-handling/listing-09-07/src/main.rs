// ANCHOR: here
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
// ANCHOR_END: here

fn main() {
    let username = read_username_from_file().expect("Unable to get username");
    println!("Username is: {}", username);
}
