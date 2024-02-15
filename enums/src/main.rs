enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move{x: i32, y: i32},
    Write(String),
    ChangeColour(i32, i32, i32),

}

impl Message {
    fn call(&self) {
        println!("message called")
    }
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let msg = Message::Write(String::from("Foo"));
    msg.call();

}
