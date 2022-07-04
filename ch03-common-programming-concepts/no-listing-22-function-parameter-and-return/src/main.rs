fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1  // K_22704 如果加上分号就是语句(不是表达式)了，语句并不会返回值，使用单位类型 () 表示不返回值
}
