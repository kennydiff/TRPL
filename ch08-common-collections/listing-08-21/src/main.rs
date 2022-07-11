fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Black"), 40);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);  // K_22711 注意这里get后的值是Option类型,可能为空
    println!("{:?}", score);
    // ANCHOR_END: here
}
