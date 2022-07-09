struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// ANCHOR: here
fn main() {
    // --snip--
    // ANCHOR_END: here

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // K_22709 示例 5-7：使用结构体更新语法为一个 User 实例设置一个新的 email 值，不过其余值来自 user1 变量中实例的字段
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // K_22709 ".." 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。 comma 逗号这里不能加，否则会报错
    };  
    // ANCHOR: here

}
// ANCHOR_END: here

