enum IpAddr {
    V4(String),
    V6(String),

}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let home = IpAddr::V4(String::from("192.168.0.140"));
    let balcoon = IpAddr::V6(String::from("::1"));

    let some = Message::Move { x: 6, y: 10 };
    println!("{:?}", some);
}
