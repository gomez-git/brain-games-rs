use crate::cli::ask_question_and_get_answer;

mod cli;

fn main() {
    let username = greet_user_and_ask_name();
}

fn greet_user_and_ask_name() -> String {
    println!("Welcome to the Brain Games!");

    let username = ask_question_and_get_answer("May I have your name? ");

    println!("Hello, {username}!");

    username
}
