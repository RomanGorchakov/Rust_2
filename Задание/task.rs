use std::io;

fn main() {
    println!("Введите пароль:");
    let mut password = String::new();
    
    io::stdin().read_line(&mut password)
        .expect("Не удалось прочитать строку");

    let password = password.trim();

    if password.chars().any(|c| c.is_numeric()) && 
       password.chars().any(|c| c.is_alphabetic()) {
        println!("Пароль надёжный!");
    } else {
        println!("Пароль должен содержать буквы и цифры!");
    }
}