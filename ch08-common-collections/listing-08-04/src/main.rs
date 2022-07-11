fn main() {
    // ANCHOR: here

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);   // K_22711 get(n) == [n]的索引获取值的方法

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // ANCHOR_END: here
}
