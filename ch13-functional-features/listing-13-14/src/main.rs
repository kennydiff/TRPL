fn main() {
    let v1 = vec![1, 2, 3];

    // let v1_iter = v1.iter();

    for val in v1.iter() {  // K_22717 这里直接写 v1 or v1.iter() 都可以
        println!("Got: {}", val);
    }
}
