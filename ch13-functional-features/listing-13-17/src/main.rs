fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);  // K_22717 iterators are lazy and do nothing unless consumed
}
