fn main() {
    // ANCHOR: here
    let config_max = Some(255u8); // None;
    if let Some(max ) = config_max {   // K_22710 = 是 == 类似match里的匹配  这句的意思是：如果config_max有值，那么就赋值给max,然后执行下面的代码
        println!("The maximum is configured to be {}", max);
    }
    // ANCHOR_END: here

    /*  用 if let 替代以下match的方式省略其模式. 节省代码
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    } */
}
