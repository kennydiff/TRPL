fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);  // K_22711 变量已经被移动进函数了
        // 这里 field_name 和 field_value 不再有效，
        // 尝试使用它们看看会出现什么编译错误！
    println!("{}",field_name);  // K_22711 出错
    // ANCHOR_END: here
}
