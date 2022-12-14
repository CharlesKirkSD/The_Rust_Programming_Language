struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,

}fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("mail@ubunto.com"), String::from("Charles"));

    user1.email = String::from("anotheremail@example.com");
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}