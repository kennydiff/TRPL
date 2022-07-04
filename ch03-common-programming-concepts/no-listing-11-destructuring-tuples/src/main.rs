fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;  // K_22704 解构（destructuring）

    println!("The value of y is: {}", y);
}