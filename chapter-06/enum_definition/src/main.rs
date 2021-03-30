
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}


// fn main() {
//     let some_number = Some(5);
//     let some_string = Some("a string");
//
//     let absent_number: Option<i32> = None;
// }


// #![allow(unused)]
// fn main() {
//     enum Option<T> {
//         Some(T),
//         None,
//     }
// }

// #[derive(Debug)]
//
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// impl Message {
//     fn call(&self) {
//         println!("Message to be written: {:#?}", self)
//         // method body would be defined here
//     }
// }
//
// fn main() {
//
//
//     let m = Message::Write(String::from("hello"));
//     m.call();
// }


// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct
//
// fn main() {}

// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }
//
// fn main() {}
//

// #![allow(unused)]
// fn main() {
//     struct Ipv4Addr {
//         // --snip--
//     }
//
//     struct Ipv6Addr {
//         // --snip--
//     }
//
//     enum IpAddr {
//         V4(Ipv4Addr),
//         V6(Ipv6Addr),
//     }
// }


// fn main() {
//     enum IpAddr {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
//
//     let home = IpAddr::V4(127, 0, 0, 1);
//
//     let loopback = IpAddr::V6(String::from("::1"));
// }


// fn main() {
//     enum IpAddr {
//         V4(String),
//         V6(String),
//     }
//
//     let home = IpAddr::V4(String::from("127.0.0.1"));
//
//     let loopback = IpAddr::V6(String::from("::1"));
// }



// enum IpAddrKind {
//     V4,
//     V6,
// }
//
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
//
// fn main() {
//     let _four = IpAddrKind::V4;
//     let _six = IpAddrKind::V6;
//
//     route(IpAddrKind::V4);
//     route(IpAddrKind::V6);
//     let home = IpAddr{
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//     let loopback = IpAddr{
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };
// }
//
// fn route(ip_kind: IpAddrKind) {}
