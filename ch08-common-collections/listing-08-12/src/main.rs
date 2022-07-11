fn main() {
    // ANCHOR: here
    let data = "initial contents";

    let s = data.to_string();  // K_22711 注意字符串切片的这个转换为String类的方法: to_string()

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    // ANCHOR_END: here
}
