struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}

// ANCHOR: here
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt() // K_22714 sqrt 返回一个数的平方根; powi 返回一个数的指定次幂的值
    }
}
// ANCHOR_END: here

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    println!("p.x = {}", p.y());
    // println!("{}", 2 ^ 2);  // K_22714 ^ 是按位异或，不是指定次幂
    // println!("{}", 4_i32.pow(2));  // K_22714 you must specify a concrete type for this numeric value, like `_i32`
    println!("{}", 64_f64.sqrt());  // K_22714 sqrt平方根函数要求：must specify a concrete type for this numeric value, like `_f64`
}
