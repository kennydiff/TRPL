fn main() {
    // ANCHOR: here
    use std::collections::HashMap;

    let text = "hello world wonderful world beautiful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {  // K_22711 whitespace就是空格,这个函数返回一个集合
        let count = map.entry(word).or_insert(0);  // K_22711 初始化键,还没有的话，就插入0，将内存中的这个值的地址赋给count, 注意这里count是个&mut i32的引用
        *count += 1;  // K_22711 *count为解引用,直接操作引用指针指向的变量的值. 对其+1
    }

    println!("{:?}", map);
    // ANCHOR_END: here
}
