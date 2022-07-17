// ANCHOR: here
fn main() {
    let string1 = String::from("long string is long");
    let result;
    
    {
        let string2 = String::from("xyz");  // K_22715 这句放到大括弧上面则可以编译通过
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
// ANCHOR_END: here

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
