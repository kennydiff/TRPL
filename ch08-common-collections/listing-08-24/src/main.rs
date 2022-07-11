fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);  // K_22711 entry用来检查“Blue”的键是否关联了一个值。如果没有，就插入值50

    println!("{:?}", scores);
    // ANCHOR_END: here
}
