fn main() {
    let num = Some(10u8);
    //пример 1
    if let Some(number) = num {
        println!("current number is: {}", number);
    }
    //пример 2
    let var: Option<i32> = None;
    if let Some(number) = var {
        println!("current number is: {}", number);
    } else {
        println!("no value!");
    }
}
