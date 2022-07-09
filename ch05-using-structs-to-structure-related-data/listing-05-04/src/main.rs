struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// ANCHOR: here
fn build_user(email2: String, username2: String) -> User {
    User {
        email: email2,
        username: username2,
        active: true,
        sign_in_count: 1,
    }
}
// ANCHOR_END: here

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
