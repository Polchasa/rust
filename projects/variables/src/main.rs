fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    {
        let x = x * 2;
        println!("The value of x is: {}", x);
    }
    println!("The value of x is: {}", x);


    let spaces = "     ";
    let spaces = spaces.len();
    println!("spaces count: {}", spaces);
}
