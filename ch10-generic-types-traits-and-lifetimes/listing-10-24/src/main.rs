struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");  // K_22715 split('.')返回被.分割的集合, next()返回集合的第一个元素
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("{}", i.part);
}
