fn main() {
    println!("Type scalair entier!");

    let intu8: u8 = "42".parse().expect("Not a number");
    println!("8 bit unsigned est {}", intu8);

    let intu16: u16 = "42".parse().expect("Not a number");
    println!("16 bit unsigned est {}", intu16);

    let intu32: u32 = "42".parse().expect("Not a number");
    println!("32 bit unsigned est {}", intu32);

    let intu64: u64 = "42".parse().expect("Not a number");
    println!("64 bit unsigned est {}", intu64);

    let intu128: u128 = "42".parse().expect("Not a number");
    println!("128 bit unsigned est {}", intu128);

    let inti8: i8 = "-42".parse().expect("Not a number");
    println!("8 bit signed est {}", inti8);

    let inti16: i16 = "-42".parse().expect("Not a number");
    println!("16 bit signed est {}", inti16);

    let inti32: i32 = "-42".parse().expect("Not a number");
    println!("32 bit signed est {}", inti32);

    let inti64: i64 = "-42".parse().expect("Not a number");
    println!("64 bit signed est {}", inti64);

    let inti128: i128 = "-42".parse().expect("Not a number");
    println!("128 bit signed est {}", inti128);


    println!("Type scalair virgule flottante! Default est f64");
    let floating32: f32 = 3.0; // f32
    println!("32 bit floating point {}", floating32);

    let floating64: f64 = 2.0; // f32
    println!("64 bit floating point {}", floating64);


    println!("Basic Arithmetic operations");
    // addition
    let sum = 5 + 10;
    println!("Additon 5+10= {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("Soustraction 95.5 - 4.3= {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("Multiplication 4x30= {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("Division: 56.7 / 32.2= {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("Reste de la division euclidienne: 43 % 5= {}", remainder);


    println!("Booleen!");
    let t = true;
    println!("Valeur de true: {}", t);
    let f: bool = false; // with explicit type annotation
    println!("Valeur de false: {}", f);

    println!("Le type charactere, supporte l'unicode");
    let c = 'z';
    println!("La lettre z: {}", c);
    let z = 'ℤ';
    println!("La lettre des ensenble entier relatif: {}", z);
    let heart_eyed_cat = '😻';
    println!("Un emoji: {}", heart_eyed_cat);


    println!("Les types composes");
    println!("Les tuples ont une longueur fixe, une fois declarer elle ne peut varier, les types a l'interieur aussi");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Exemple de tuple: let tup: (i32, f64, u8) = (500, 6.4, 1) - Valeur des elements apres destructure x:{}  y:{} z:{}", x, y, z);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;
    println!("Le premier element via tup.0 : {}", five_hundred);
    let six_point_four = tup.1;
    println!("Le second element via tup.1 : {}", six_point_four);
    let one = tup.2;
    println!("Le troisieme element via tup.3 : {}", one);


    println!("Le type tableau ont une longeur fixe, doit toujours contenir le meme type d'element");
    let _a = [1, 2, 3, 4, 5];

    println!("Tableau: let a = [1, 2, 3, 4, 5];");

    let _months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Tableau: let a: [i32; 5] = [1, 2, 3, 4, 5];");
    let _a = [3; 5];
    let _a = [3, 3, 3, 3, 3];
    println!("Tableau: let a = [3; 5]; equivalent a let a = [3, 3, 3, 3, 3];");

    let a = [1, 2, 3, 4, 5];
    println!("Tableau: let a = [1, 2, 3, 4, 5];");
    let first = a[0];
    let second = a[1];
    println!("Acces via a[0]  {}", first);
    println!("Acces via a[1]  {}", second);

//    ERROR
//    let index = 10;
//    let element = a[index];
//    println!("The value of element is: {}", element);
//    thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:114:19
//    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

}
