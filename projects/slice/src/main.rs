fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    let word2 = first_word2(&s);      
    println!("Word2 is: {word2}");
    s.clear();
    println!("Word end's at: {word}");
    
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];

    for i in slice {
        println!("{i}");
    }
}

fn first_word(s: &String) -> usize {
    //func return end index of first word in string - возвращает индекс конца первого слова в строке
    let bytes = s.as_bytes(); //преобразование String в массив байтов
                              //цикл. iter - возвращает итератор, enumerate - оборачивает iter и возвращает индекс и ссылку на элемент. (pair в c++)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //ищем байт (b) равный пробелу ' '
            return i; //нашли - возвращаем его индекс
        }
    }

    s.len() //иначе вернем длину строки подразумевя что вся строка из одного слова.
}

fn first_word2(s: &String) -> &str {
    //&str - возвращаем срез строки
    let bytes = s.as_bytes(); //преобразование String в массив байтов
                              //цикл. iter - возвращает итератор, enumerate - оборачивает iter и возвращает индекс и ссылку на элемент. (pair в c++)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            //ищем байт (b) равный пробелу ' '
            return &s[0..i]; //нашли - возвращаем его индекс
        }
    }

    &s[..] //иначе вернем длину строки подразумевя что вся строка из одного слова.
}
