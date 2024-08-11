#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };

    let r2 = Rectangle {
        width: 20,
        height: 30,
    };

    let r3 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("r1 area: {}, r2 area: {}, r3 area: {}.", r1.area(), r2.area(), r3.area());
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r3 hold r1? {}", r3.can_hold(&r1));
}