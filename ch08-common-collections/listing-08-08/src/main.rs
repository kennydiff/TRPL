fn main() {
    // ANCHOR: here
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;  // K_22711 "*" 的意思是从引用(内存指针)中取出值，然后再把值加上50，然后再把值写回到引用(内存指针)中。
    }

    for b in v {  // K_22711
        println!("{}", b);
    }

    // ANCHOR_END: here
}
