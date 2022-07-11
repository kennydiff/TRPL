fn main() {
    // ANCHOR: here
    let s1 = String::from("hello");
    // let s1 = "hello";
    let h = s1[0];  // K_22711 不论是String or &str 都无法通过数组索引的方式访问其中的某个字符
                                                    // 因为大多数国家的字符(utf-8)都不止一个字节
    // ANCHOR_END: here
}
