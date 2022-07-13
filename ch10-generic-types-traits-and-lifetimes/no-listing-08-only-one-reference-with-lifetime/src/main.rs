fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// ANCHOR: here
fn longest<'a>(x: &'a str, y: &str) -> &'a str {  // K_22713 y 的生命周期与参数 x 和返回值的生命周期没有任何关系，所以y不必加声明周期声明
    x
}
// fn longest<'a>(x: &'a str, y: &str) -> &'a str {
//     x
// }
// ANCHOR_END: here
