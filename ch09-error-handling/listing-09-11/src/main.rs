// ANCHOR: here
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()  // K_22713 next()? 这里可能返回None ，所以要用?来解决错误往上传递的问题,否则就要match来写
}
// ANCHOR_END: here

fn main() {
    assert_eq!(
        last_char_of_first_line("Hello, world\nHow are you today?"),
        Some('d')
    );

    assert_eq!(last_char_of_first_line(""), None);
    assert_eq!(last_char_of_first_line("\nhi"), None);
}
