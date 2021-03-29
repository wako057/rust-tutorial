struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: false,
        sign_in_count: 1,
    };

    print_user(&user1);

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("anotheremail@example.com");
    print_user(&user2);

    let user3 = build_user(
        String::from("wako057@gmail.com"),
        String::from("Wako awax")
    );
    print_user(&user3);

    let user4 = build_user_shorthand(
        String::from("wako057@hotmail.com"),
        String::from("Wako mini")
    );
    print_user(&user4);

    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    print_user(&user4);

    let user5 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    print_user(&user5);
    // drop(user1);
    // print_user(&user5);



    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    print_color(black);
    print_origin(origin);

}

fn print_origin(origin: Point)
{
    println!("Point Values   X: [{}]   Y: [{}]   Z: [{}]", origin.0, origin.1, origin.2);
}


fn print_color(color: Color)
{
    println!("Colors Values  R: [{}]   G: [{}]   B: [{}]", color.0, color.1, color.2);
}

fn print_user(user: &User)
{
    println!("user.email: [{}]\nuser.username: [{}]\nuser.active: [{}]\nuser.sign_incount: [{}]", user.email, user.username, user.active, user.sign_in_count);
    println!("############################################");
}
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

