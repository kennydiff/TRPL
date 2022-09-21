struct Cacher<T>
where
    T: Fn(u32) -> u32,  // K_22717 所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个
{
    calculation: T,  // K_22717 T 是一个使用<Fn>的闭包
    value: Option<u32>,
}

fn main() {}
