use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;  // K_22713 main 函数的返回类型是 () 而不是 Result，所以会报错
}
