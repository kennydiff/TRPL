use adder;

// #[cfg(test)]  // K_22716 这句话可以不要，tests里的所有rs文件都会被当作集成测试文件来对待

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
