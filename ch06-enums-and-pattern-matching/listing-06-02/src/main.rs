// ANCHOR: here
enum Message {
    Quit2,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// ANCHOR_END: here

fn main() {
    let a = Message::Move{x: 3, y: 4};
}
