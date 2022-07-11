fn main() {
    // ANCHOR: here
    // let mut v = Vec::new();
    let mut v = vec![1, 2, 3,4];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:#?}", v);
    // ANCHOR_END: here
}
