#[derive(Debug)]
struct User {
    email: String,
    password: String,
    id: u32,
}

impl User {
    fn repr(&self) -> () {
        println!("User email is {} and id is {}.", self.email, self.id);
    }
}

struct Color(u32, u32, u32);

fn main() {

    let user = User {
        email: String::from("user1"),
        password: String::from("pas1"),
        id: 1,
    };

    let user2 = User {
        id: 2,
        ..user
    };

    user2.repr();

    let clr1 = Color(1, 2, 3);
    println!("{}", clr1.0)
}
