fn main() {
    let string = no_dangle();
    println!("{}", string);
}

// ANCHOR: here
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
// ANCHOR_END: here
