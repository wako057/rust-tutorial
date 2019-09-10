fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_arg(5);
    another_function_with_args(5, 6);

    let x = 5;

    println!("Ecriture particuliere d'un bloc aui sera evaluer grace au manque du point virgule");
    let y = {
        let x = 3;
        x + 1
    };

    println!("Final: The value of y is: {}", y);
    println!("Final: The value of x is: {}", x);

    let x = five();

    println!("five-The value of x is: {}", x);

    let x = plus_one(5);

    println!("plus_one-The value of x is: {}", x);

    let x = classic_return(50);
    println!("classic_return-The value of x is: {}", x);
}
fn classic_return(x: i32) -> i32{
    return x + 22;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_arg(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function_with_args(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
