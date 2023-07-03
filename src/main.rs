use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to the Brain Games!");

    print!("May I have your name? ");

    io::stdout().flush().unwrap();

    let mut username = String::new();

    io::stdin()
        .read_line(&mut username)
        .expect("Something bad happened...");

    let username = username.trim();

    println!("Hello, {username}!");
}
