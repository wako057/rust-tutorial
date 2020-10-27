fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    // KO on ne peut pas changer une variable meme par reference si elle est immuable
    //  change_fail(&s);

//    KO On ne peut avoir qu'une seul reference mutable par entite
//    let mut s = String::from("hello");
//    let r1 = &mut s;
//    let r2 = &mut s;
//    println!("{}, {}", r1, r2);
    println!("{}", s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn change_fail(some_string: &String) {
//     some_string.push_str(", world");
// }
