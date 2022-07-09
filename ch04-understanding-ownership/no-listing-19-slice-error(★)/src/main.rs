fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// ANCHOR: here
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    

    s.clear(); // error!
    
    println!("the first word is: {}", word);  // K_22708 这行改放到s.clear()之前的话，就不报错了

}
// ANCHOR_END: here
