fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);  // K_22717 调用闭包两次，第一次使用 String 类型作为参数而第二次使用 u32，则会报错
}
