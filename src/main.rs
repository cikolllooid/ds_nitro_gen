use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};
use std::fs::File;

async fn send_request(password: &str) -> Result<(), reqwest::Error> {
    let url = format!("https://discord.com/api/v8/entitlements/gift-codes/{}", password);
    let good_url = format!("https://discord.gift/{}", password);
    let response = reqwest::get(&url).await?;
    let status = response.status();

    if status.is_success() {
        print!("Valid code: https://discord.gift/{}", password);
        let mut file = File::create("Valid.txt").expect("Не удалось создать файл");
        writeln!(file, "{}", good_url).expect("Не удалось записать в файл");
    } else {
        println!("Invalid code: https://discord.gift/{}", password);
    }

    Ok(())
}

fn podbor(num_passwords: usize) -> Vec<String> {
    let charset: Vec<char> = (b'0'..=b'9')
        .chain(b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .map(|c| c as char)
        .collect();

    let mut passwords = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..num_passwords {
        let password: String = (0..16)
            .map(|_| *charset.choose(&mut rng).unwrap())
            .collect();
        passwords.push(password);
    }
    passwords
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    print!("Введите количество паролей для генерации: ");
    io::stdout().flush().expect("Не удалось сбросить буфер вывода");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");
    let num_passwords: usize = input.trim().parse().expect("Не удалось преобразовать в число");

    let passwords = podbor(num_passwords);
    for password in passwords {
        send_request(&password).await?;
    }
    print!("Нажмите Enter для выхода...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    Ok(())
}
