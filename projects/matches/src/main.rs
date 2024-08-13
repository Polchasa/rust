//Пример 1
enum Coins {
    Penny,
    Nickel,
    Dime,
    Quarter,    
}

fn value_in_cents (coin: Coins) -> u8 {
    match coin {
        Coins::Penny => 1,
        Coins::Nickel => 5, // если coin будет соответсвовать типу Coins::Nickel то возвращаем 5
        Coins::Dime => 20,
        Coins::Quarter => 25,
    }
}
//Пример 1 - конец
//Пример 2 - начало
fn value_in_cents_2 (coin: Coins) -> u8 {
    match coin {
        Coins::Penny => {
            println!("it's a penny! sss");
            1
        }
        Coins::Nickel => 5, // если coin будет соответсвовать типу Coins::Nickel то возвращаем 5
        Coins::Dime => 20,
        Coins::Quarter => 25,
    }
}
//Пример 2 - конец
//Пример 3 - начало
#[derive(Debug)]
enum States{
    Alabama,
    Alaska,
    // another states here
}
enum Coins2 {
    Penny,
    Nickel,
    Dime,
    Quarter(States), 
}
fn value_in_cents_3 (coin: Coins2) -> u8 {
    match coin {
        Coins2::Penny => 1,
        Coins2::Nickel => 5, // если coin будет соответсвовать типу Coins::Nickel то возвращаем 5
        Coins2::Dime => 20,
        Coins2::Quarter(State) => {
            println!("Quarter state is: {State:?}");
            25
        }
    }
}
//Пример 3 - конец
fn main() {
    println!("{}", value_in_cents(Coins::Nickel));
    value_in_cents_2(Coins::Penny);
    value_in_cents_3(Coins2::Quarter(States::Alabama));    
}
