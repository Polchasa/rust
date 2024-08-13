#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),

}

impl IpAddr {
    fn get_ip(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let home = IpAddr::V4(String::from("192.168.0.140"));
    let balcoon = IpAddr::V6(String::from("::1"));
    home.get_ip();

    let some_num = Some(5);
    println!("{:?}", some_num);
}
