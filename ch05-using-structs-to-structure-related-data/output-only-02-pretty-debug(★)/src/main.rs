#[derive(Debug)]  // K_22708 派生 Debug 子属性
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);  // K_22708 注意{:#?}和{:?}的区别
}
