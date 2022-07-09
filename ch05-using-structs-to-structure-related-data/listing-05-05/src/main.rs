struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}



fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}


// ANCHOR: here
fn build_user(email: String, username: String) -> User {
    User {
        email,  // K_22708 同名参数可以省去这样的写法：email: email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
// ANCHOR_END: here