struct User {
    username: String,
    email: String,
    active: bool,
}

fn main() {

    let mut user1 = User {
        email: String::from("someone@x.com"),
        username: String::from("someone"),
        active: true,
    };

    let name = user1.username;
    user1.username = String::from("anotherone");

    let user2 = build_user(String::from("someoneelse@gmail.com"), String::from("someoneelse"));

    let user3 = User {
        email: String::from("elon@gmail.com"),
        username: String::from("elon"),
        ..user2
    };
    
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
    }
}
