pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// ANCHOR: here
pub fn notify(item: &impl Summary) {  // K_22713 traits作为参数的时候，需要加上 &impl，意思是实现了这个trait的所有类型都可以传进来
    println!("Breaking news! {}", item.summarize());
}
// ANCHOR_END: here
