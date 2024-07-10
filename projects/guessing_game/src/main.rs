use rand::Rng; //рандом
use std::cmp::Ordering; //для сравнения числа с числом (больше меньше равно)
use std::io; //ввод вывод

fn main() {
    println!("Guess the number!"); //вывели название игры

    let secret_number = rand::thread_rng().gen_range(1..=100); //сгенерировали число от 1 до 100

    //println!("Secret number is: {secret_number}"); //вывод сгенерированного числа для отладки

    loop { //запускаем цикл для угадывания числа
        println!("Please input your guess."); //просим пользователя ввести число

        let mut guess = String::new(); //подготовили изменяемую переменную типа строка для сохранения ввода пользователя

        io::stdin() //считываем введеное число
            .read_line(&mut guess) //считываем ввод пользователя в переменную, передаем изменяемую ссылку на переменную
            .expect("Failed to read line"); //сообщение на случай ошибки

        let guess: u32 = match guess.trim().parse() { //парсим из строки число безнакового типа
            Ok(num) => num, //если все ок сохраняем?
            Err(_)  => { //если ошибка
                println!("Not number! Try again."); //даем сообщение
                continue; //продолжаем выполнение программы
            }
        };

        println!("You guessed: {}", guess); //что ввел пользователь

        match guess.cmp(&secret_number) { //делаем операцию сравнения введеного числа и сгенерированного
            Ordering::Less => println!("Too small"), //введенное меньше сгенерированного
            Ordering::Greater => println!("Too big"), //введенное больше сгенерированного
            Ordering::Equal => { //введенное равно сгенерированному
                println!("You win!"); //сообщаем о победе
                break; //завершаем цикл
            }
        }
    }
}
