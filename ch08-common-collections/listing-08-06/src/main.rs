fn main() {  // K_22711 这个范例演示了"不能在相同作用域中同时存在可变和不可变引用的规则" 
    // ANCHOR: here
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];  // first为引用，导致v的push操作无法继续

    v.push(6);

    println!("The first element is: {}", first);
    // ANCHOR_END: here
}
