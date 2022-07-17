pub struct Guess {
    value: i32,
}

// ANCHOR: here
// --snip--

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]  // K_22716 - should_panic 添加期望信息，没有出现期望信息都算作Fail
    fn greater_than_100() {
        Guess::new(101);
        // Guess::new(0);
    }
}
// ANCHOR_END: here
