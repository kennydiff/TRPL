fn main() {
    // ANCHOR: here
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}  // K_22710 main()函数内的函数??? ...
    fn remove_fancy_hat() {}
    // ANCHOR_END: here
}
