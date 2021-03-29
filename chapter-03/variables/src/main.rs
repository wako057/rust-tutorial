fn main() {
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The number of spaces: {}", spaces);

    const MAX_POINTS: u32 = 100_000;

    println!("The number of the constant MAX_POINTS: {}", MAX_POINTS);

}
