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

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    println!("Home IP Addr: {:?}", home);
    println!("Loopback: {:?}", loopback);

    impl Message {
        fn call (&self) {
            println!("MESSAGE: {:?}", self);
        }        
    }

    let m = Message::Write(String::from("Hello World!"));
    m.call();

    let some_num = Some(5);
    let absent_num: Option<i32> = None;
}