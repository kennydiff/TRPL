#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())  // K_22715 注意Ok(())的写法
        } else {
            Err(String::from("2+2不等于4..."))  // K_22715 two plus two does not equal four
        }
    }
}
