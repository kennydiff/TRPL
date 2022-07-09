#[derive(Debug)]
struct Rectangle {  // K_22708 矩形
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),  // K_22708 dbg!是宏, 它会在调试时打印出变量的值(dirty debug)
        height: 50,
    };

    dbg!(&rect1);
}
