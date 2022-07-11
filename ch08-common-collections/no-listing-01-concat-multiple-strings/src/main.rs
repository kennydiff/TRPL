fn main() {
    // ANCHOR: here
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;  // K_22711 注意第一个s1不用引用
    println!("{}", s);
    // ANCHOR_END: here
}
