fn main() {
    // ANCHOR: here
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
            // None => None,  // K_22710 没有这句话的话，则Match覆盖不完整,会报错~
        }
    }
    // ANCHOR_END: here

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
