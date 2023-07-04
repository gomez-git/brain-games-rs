use std::io;
use std::io::Write;

fn main() {
    let username = greet_user_and_ask_name();
}

fn greet_user_and_ask_name() -> String {
    println!("Welcome to the Brain Games!");

    print!("May I have your name? ");

    io::stdout().flush().unwrap();

    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Something bad has happened...");

    let username = String::from(username.trim());

    println!("Hello, {username}!");

    username
}
