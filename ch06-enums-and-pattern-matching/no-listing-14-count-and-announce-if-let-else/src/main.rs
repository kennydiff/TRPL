#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    // ANCHOR: here
    let mut count = 0;
    if let Coin::Quarter(state) = coin {  // K_22710 替代刚刚match的写法...看着简洁很多: 如果匹配则赋值，否则执行else逻辑
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    // ANCHOR_END: here
}
