use super::cli::ask_question_and_get_answer;
use crate::games::brain_dice::BrainDice;
use crate::games::brain_even::BrainEven;
use crate::games::StartGame;
use std::process;

pub struct User {
    name: String,
}

impl User {
    fn new(name: String) -> Self {
        Self { name }
    }

    pub fn greet() -> Self {
        println!("Welcome to the Brain Games!");

        let name = ask_question_and_get_answer("May I have your name? ");

        println!("Hello, {name}");

        Self::new(name)
    }

    pub fn choose_game(&self) -> Box<dyn StartGame> {
        let user_choice = ask_question_and_get_answer("Your choice: ");

        match user_choice.as_str() {
            "f" => Box::new(BrainEven::new()),
            "d" => Box::new(BrainDice::new()),
            _ => {
                self.say_bye();
                process::exit(0);
            }
        }
    }

    pub fn react_on_result(&self, result: bool) {
        match result {
            true => println!("Congratulations, {}!", self.name),
            false => println!("Let's try again, {}!", self.name),
        };
    }

    fn say_bye(&self) {
        println!("Goodbye, {}.", self.name);
    }
}
