use super::StartGame;
use crate::math::find_greatest_common_divisor;

pub struct BrainGreatestCommonDivisor {
    task: &'static str,
}

impl BrainGreatestCommonDivisor {
    pub fn new() -> Self {
        Self {
            task: "Find the greatest common divisor of given numbers.",
        }
    }
}

impl StartGame for BrainGreatestCommonDivisor {
    fn task(&self) -> &str {
        self.task
    }

    fn get_args(&self) -> Vec<u8> {
        vec![
            self.get_random_number(1, 100),
            self.get_random_number(1, 100),
        ]
    }

    fn get_correct_answer(&self, args: Vec<u8>) -> String {
        let num1 = args[0];
        let num2 = args[1];

        find_greatest_common_divisor(num1, num2).to_string()
    }
}
