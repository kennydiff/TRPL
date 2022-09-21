fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];  // K_22717 Vec 动态数组

    // let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();  // K_22717 `collection()` Transforms an iterator into a collection
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();  // K_22717 Vec<_> 里的 `_` 是任意类型，相当于类型占位符，由编译器推断出具体类型 `map()` transforms one iterator into another
    assert_eq!(v2, vec![2, 3, 4]);
}
