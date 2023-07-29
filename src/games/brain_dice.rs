use super::StartGame;
use std::str;

pub struct BrainDice {
    task: &'static str,
}

impl BrainDice {
    pub fn new() -> Self {
        Self {
            task: "A dice has been thrown... Try to guess the number from 1 to 6.",
        }
    }
}

impl StartGame for BrainDice {
    fn task(&self) -> &str {
        self.task
    }

    fn question(&self, _: &[u8]) -> String {
        "*".to_string()
    }

    fn get_args(&self) -> Vec<u8> {
        vec![self.get_random_number(1, 6)]
    }

    fn get_correct_answer(&self, args: Vec<u8>) -> String {
        args[0].to_string()
    }
}
