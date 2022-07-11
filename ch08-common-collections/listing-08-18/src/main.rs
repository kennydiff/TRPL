fn main() {
    // ANCHOR: here
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
                       // ANCHOR_END: here
    println!("{}", s1);  // K_22711 s1 在上一行已经被借走了，不能再被使用了，但是s2还可以被使用
    println!("{}", s2);
}
