#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let r = Rectangle {
        width: dbg!(30 * 555),
        height: 50,
    };
    dbg!(&r);
    println!("Rectangle area is: {}", area(&r))
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}