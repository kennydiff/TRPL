use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");  // K_22713 expect 与 unwrap 类似,但是可以自定义错误的时候Panic提示的信息
}
