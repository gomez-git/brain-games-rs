use super::cli::ask_question_and_get_answer;

pub struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        Self { name }
    }

    pub fn greet() -> Self {
        println!("Welcome to the Brain Games!");

        let question = "May I have your name? ";
        let name = ask_question_and_get_answer(question);

        println!("Hello, {name}");

        Self::new(name)
    }
}
