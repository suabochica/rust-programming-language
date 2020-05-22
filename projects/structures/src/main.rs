fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    };

    let user_one_immutable = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user_one_mutable = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    user.email = String::from("anotheremail@example.com");

    let user_two_immutable = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user_one_immutable.active,
        sign_in_count: user_one_immutable.sign_in_count,
    }
}
