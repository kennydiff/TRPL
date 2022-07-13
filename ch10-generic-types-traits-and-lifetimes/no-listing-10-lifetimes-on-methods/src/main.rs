#[derive(Debug)]
struct ImportantExcerpt{
    // part: &'a str,  // K_22713 这里因为用到了引用，所以必须要加上声明周期声明
    part_s: String  // K_22713 把这里成员从引用(字符串切片)改为了String类型,则不需要声明周期声明了!
}

// ANCHOR: 1st
// impl<'a> ImportantExcerpt<'a> {
//     fn level(&self) -> i32 {
//         3
//     }
// }
// ANCHOR_END: 1st

// ANCHOR: 3rd
// impl<'a> ImportantExcerpt<'a> {
//     fn announce_and_return_part(&self, announcement: &str) -> &str {
//         println!("Attention please: {}", announcement);
//         self.part
//     }
// }
// ANCHOR_END: 3rd

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part_s: first_sentence.to_string(),
    };
    println!("{:?}", i.part_s);
}
