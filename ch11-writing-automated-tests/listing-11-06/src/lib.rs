#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// ANCHOR: here
#[cfg(test)]  // K_22716 cfg(test)的标签放到 mod 的前面
mod tests {
    use super::*;

    #[test] // K_22716 test标签放到被测试的函数的前面
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
// ANCHOR_END: here
