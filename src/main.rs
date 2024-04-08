#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("Home IP Addr: {:?}", home);
    println!("Loopback: {:?}", loopback);


    let m = Message::Write(String::from("Hello World!"));
    let c = Message::ChangeColor(40, 10, 30);
    exec_message(m);
    exec_message(c);

    let some_num = Some(5);
    let absent_num: Option<i32> = None;

    println!("Some num(5) plus 1 equals {:?}", plus_one(some_num));
    plus_one(absent_num); // None case is controlled, so no error occurs.

    let quarter = Coin::Quarter;
    println!("Value of a quarter: {}", value_in_cents(quarter));

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let message = Message::Quit;
    if let Message::Write(string) = message {
        println!("MESSAGE: {}", string);
    } else{
        println!("NOT A WRITTEN MESSAGE.")
    }
}

fn exec_message(message: Message) {
    match message {
        Message::Quit => println!("QUITTING."),
        Message::Move{x, y} => println!("MOVING X: {} , Y: {}", x, y),
        Message::Write(string) => println!("MESSAGE: {}", string),
        Message::ChangeColor(x, y, z) => println!("COLOR VALUE: {} {} {}", x, y, z),
    }
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}