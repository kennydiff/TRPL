pub struct Guess {
    value: i32,
}

// ANCHOR: here
// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        // if value < 1 {
        if value < 1 || value > 100 {  // K_22715 在范围之外就Panic
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
// ANCHOR_END: here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]  // K_22715 Should panic , 没有Panic的话就是测试失败.
    fn greater_than_100() {
        Guess::new(200);
    }
}
