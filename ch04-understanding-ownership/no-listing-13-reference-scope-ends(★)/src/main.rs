fn main() {
    // ANCHOR: here
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);  // K_22708 注意一个引用的作用域从声明的地方开始一直持续到最后一次使用为止,这里r1/r2后面不被调用/借用/访问，就没问题，但是如果发生的读取就会报错.
    // 这里r1,r2最后一次使用，作用域结束, variables r1 and r2 will not be used after this point    

    let r3 = &mut s; // no problem
    println!("{}", r3);  // K_22708 最后一次使用r3，作用域结束
    // println!("{} and {}", r1, r2);    
    let r4 = &mut s; // no problem
    println!("{}", r4);  // K_22708 最后一次使用r4，作用域结束
    // println!("{}, {}", r1, r2);
    // ANCHOR_END: here
}
