fn largest<T>(list: &[T]) -> &T 
where T: PartialOrd + Copy  // K_22714 要添加这个限制条件，否则范型范围太广，很多没有实现PartialOrd 或 Copy的类型就无法执行以下函数...
{
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    // let char_list = vec![3.5, 4.6, 7.3, 1.3];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
