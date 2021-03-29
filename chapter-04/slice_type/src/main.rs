fn main() {
    // let mut s = String::from("Hello, world!");
    let s = String::from("Hello, world!");
    println!("{}", s);
    let result = first_word(&s);
    println!("Chaine: [{}] result: [{}]", s, result);
    // s.clear();
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Chaine hello: [{}] Chaine world: [{}]", hello, world);


    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("word: [{}]", word);
    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("word: [{}]", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word: [{}]", word);


    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
    println!("Tableau integer: {:?} Slice integer: {:?}", a, slice); // permit grace a #[derive(Debug)]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
