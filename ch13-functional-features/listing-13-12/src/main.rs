fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;  // K_22717 equal_to_x 闭包也被允许使用变量 x，因为它与 equal_to_x 定义于相同的作用域

    let y = 4;

    assert!(equal_to_x(y));
}
