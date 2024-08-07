fn main() {
    //зададим переменную по которой будем определять условие
    let condition: bool = true;

    //переменная будет определена по результатам блока if else потому что if является выражением,
    //значения в фигурных скобках обязательно должны быть одного типа
    let number: i32 = if condition { 5 } else { 10 };
    // ну и классический if, else if, else Только само условие не в круглых скобках.
    if number != 5 {
        println!("its not five");
    } else if number == 10 {
        println!("its ten");
    } else {
        println!("maybe its five");
    }
}
