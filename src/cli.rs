use std::io;
use std::io::Write;

pub fn ask_question_and_get_answer(question: &str) -> String {
    print!("{question}");

    io::stdout().flush().unwrap();

    let mut user_answer = String::new();

    io::stdin()
        .read_line(&mut user_answer)
        .expect("Something bad has happened...");

    user_answer.trim().to_string()
}
