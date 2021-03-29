fn main() {
    println!("Hello, world!");
    let s = "hello";
    println!("Type str: {}", s);
    let s = String::from("hello");
    println!("Type String: {}", s);
    let mut s = String::from("hello");
    println!("s: {}", s);
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("s: {}", s);

    let mut x = 5;
    let y = x;
    x = 6;
    println!("x: {} Y: {}", x, y);

    let mut s1 = String::from("hello");
    let s2 = s1.clone();
    s1.push_str(", blublu");
    println!("s1: {} s2:  {}", s1, s2);

    let s = String::from("hello");  // s comes into scope
    println!("s: {}", s);
    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here
//    println!("s: {}", s); // KO ne marche pas: value borrowed here after move
    let x = 5;                      // x comes into scope
    println!("x: {}", x);
    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward
    println!("x: {}", x); // OK

    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1
    println!("s1: {}", s1); // OK
    let s2 = String::from("hello");     // s2 comes into scope
    println!("s2: {}", s2);
    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    println!("s1: {}", s1);
//    println!("s2: {}", s2); // value borrowed here after move
    println!("s3: {}", s3);
    // takes_and_gives_back, which also
    // moves its return value into s3

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
