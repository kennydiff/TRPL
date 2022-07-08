fn main() {
    for number in (1..100).rev() {  // K_22705 这里小括号的表示是一个区间，而不是一个数字
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
