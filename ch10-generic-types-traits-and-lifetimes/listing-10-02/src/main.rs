fn main() {
    let number_list = vec![34, 50, 125, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number_A is {}", largest);

    let number_list = vec![102, 34, 6000, 8964, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number_B is {}", largest);
}
