use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();  // K_22713  Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
}
