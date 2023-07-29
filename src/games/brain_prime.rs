use super::StartGame;
use crate::math::check_primeness;

pub struct BrainPrime {
    task: &'static str,
}

impl BrainPrime {
    pub fn new() -> Self {
        Self {
            task: "Answer 'yes' if given number is prime. Otherwise answer 'no'.",
        }
    }
}

impl StartGame for BrainPrime {
    fn task(&self) -> &str {
        self.task
    }

    fn get_correct_answer(&self, args: Vec<u8>) -> String {
        match check_primeness(args[0]) {
            true => "yes".to_string(),
            false => "no".to_string(),
        }
    }
}
