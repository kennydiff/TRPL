// ANCHOR: here
fn main() {
    let number_list = vec![34, 50, 100, 90, 95];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    // ANCHOR_END: here
    assert_eq!(*largest, 100);  // K_22714 不等于100就Panic? ... 这啥逻辑?
    // ANCHOR: here
}
// ANCHOR_END: here
