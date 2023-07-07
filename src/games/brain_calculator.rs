use super::super::math::calculator;
use super::StartGame;
use std::str;

const OPERATORS: [u8; 4] = [b'+', b'-', b'*', b'%'];

pub struct BrainCalculator {
    task: &'static str,
}

impl BrainCalculator {
    pub fn new() -> Self {
        Self {
            task: "What is the result of the expression?",
        }
    }
}

impl StartGame for BrainCalculator {
    fn task(&self) -> &str {
        self.task
    }

    fn get_args(&self) -> Vec<u8> {
        let num1 = self.get_random_number(1, 100);
        let num2 = self.get_random_number(1, 100);
        let index = self.get_random_number(0, 3) as usize;
        let operator = OPERATORS[index];

        vec![num1, operator, num2]
    }

    fn question(&self, args: &Vec<u8>) -> String {
        let binding = [args[1]];
        let operator = str::from_utf8(&binding).unwrap();

        format!("{} {} {}", args[0], operator, args[2])
    }

    fn get_correct_answer(&self, args: Vec<u8>) -> String {
        let num1 = args[0] as i16;
        let operator = args[1];
        let num2 = args[2] as i16;

        calculator(num1, operator, num2).to_string()
    }
}
