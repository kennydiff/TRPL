#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// ANCHOR: here
// --snip--
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // self.width < other.width && self.height > other.height  // K_22715 这里宽小于另一个的宽，是故意引入的一个bug,应该宽大于宽，高大于高才能放下另一个长方形
        self.width > other.width && self.height > other.height
    }
}
// ANCHOR_END: here

// let larger = Rectangle {
//     width: 8,
//     height: 7,
// };
// let smaller = Rectangle {
//     width: 5,
//     height: 1,
// };

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
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

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
