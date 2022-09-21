use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        // K_22717 定义函数指针指向一个闭包
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    fn add_one_v1(x: u32) -> u32 {  // K_22717 函数定义
        x + 1
    }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };  // K_22717 完整标注的闭包定义
    let add_one_v3 = |x| {x + 1_f64};
    let add_one_v4 = |x| x + 1_f64;  // K_22717 因为闭包体只有一行，省略了可选的大括号
    
    let sd3 =  add_one_v3(7.1);
    println!("{}", sd3);

    let sd4 =  add_one_v4(15.4);
    println!("{}", sd4);
    // let sc =  add_one_v4(5);
}
