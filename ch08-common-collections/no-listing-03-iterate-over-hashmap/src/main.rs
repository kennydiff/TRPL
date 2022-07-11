

fn main() {
    // ANCHOR: here
    use std::collections::HashMap;  // K_22711 这个use放到main()之外也没问题的

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // ANCHOR_END: here
}
