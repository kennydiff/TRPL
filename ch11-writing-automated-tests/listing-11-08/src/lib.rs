pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]  // K_22716 - should_panic 这个标签表示：如果发生了Panic就是我们想要的结果，通过测试.
    fn greater_than_100() {
        Guess::new(200);
    }
}
