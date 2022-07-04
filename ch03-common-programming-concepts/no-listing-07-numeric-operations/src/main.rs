fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);
    // K_22704 C/C++都类似，当被除数和除数都是整数时，并不会得到一个double的浮点型的数，而是直接舍去小数部分（即向下取整）
    let floored = 2 / 3; // Results in 0 because 2/3 = 0.6666... is floored to 0.0
    println!("floored: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);
}
