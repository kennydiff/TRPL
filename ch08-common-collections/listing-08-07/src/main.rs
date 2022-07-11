fn main() {
    // ANCHOR: here
    let v = vec![100, 32, 57];
    for i in &v {  // K_22711 这里v是否引用&都可以的
        println!("{}", i);
    }
    // ANCHOR_END: here
}
