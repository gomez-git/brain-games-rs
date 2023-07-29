use crate::cli::ask_question_and_get_answer;
use rand::Rng;

pub mod brain_calculator;
pub mod brain_dice;
pub mod brain_even;
pub mod brain_gcd;
pub mod brain_prime;
pub mod brain_progression;

pub trait StartGame {
    fn task(&self) -> &str;

    fn get_random_number(&self, start: u8, end: u8) -> u8 {
        rand::thread_rng().gen_range(start..=end) as u8
    }

    fn get_args(&self) -> Vec<u8> {
        vec![self.get_random_number(1, 100)]
    }

    fn question(&self, args: &Vec<u8>) -> String {
        args.iter()
            .map(|elm| elm.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn get_correct_answer(&self, args: Vec<u8>) -> String;

    fn run(&self) -> bool {
        println!("{}", self.task());

        let mut round: u8 = 1;

        while round <= 3 {
            let args = self.get_args();

            println!("Question: {}", self.question(&args));

            let user_answer = ask_question_and_get_answer("Your answer: ");
            let correct_answer = self.get_correct_answer(args);

            match user_answer == correct_answer {
                true => {
                    println!("Correct!");

                    round += 1;
                }
                false => {
                    println!(
                        "'{user_answer}' is wrong answer ;(. Correct answer was '{correct_answer}'.",
                    );
                    return false;
                }
            }
        }

        true
    }
}
