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
//Пример 4 - начало
fn plus_one(x: Option<i32>) -> Option<i32> { //функция ожидает от нас переменную с типом Option<i32>
    match x { //Проверяем тип
        None => None, //Если встречен None его и возращаем
        Some(i) => Some(i + 1), //Если встречен тип Option<i32>, берем значение этой переменной 
                                     //Добавим еденицу и вернем это значение из функции
    }
} 
//Пример 4 - конец
fn main() {
    println!("{}", value_in_cents(Coins::Nickel));
    value_in_cents_2(Coins::Penny);
    value_in_cents_3(Coins2::Quarter(States::Alabama));   
    println!("{:?}", plus_one(Some(5))); 
    println!("{:?}", plus_one(None)); 
    //Пример 5 - начало
    let dice_roll = 9;    
    match dice_roll {
        7 => println!("have hat"),
        3 => println!("hat out"),
        other => println!("player pos changed on {}", other),
    }
    //Пример 5 - конец
    //Пример 6 - начало
    let dice_roll = 9;    
    match dice_roll {
        7 => println!("have hat"),
        3 => println!("hat out"),
        _ => println!("Roll again!!!"),
    }
    //Пример 6 - конец
    //Пример 7 - начало
    let dice_roll = 9;    
    match dice_roll {
        7 => println!("have hat"),
        3 => println!("hat out"),
        _ => (),
    }
    //Пример 7 - конец
}