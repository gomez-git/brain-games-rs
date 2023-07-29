use super::StartGame;
use crate::math::check_evenness;

pub struct BrainEven {
    task: &'static str,
}

impl BrainEven {
    pub fn new() -> Self {
        Self {
            task: "Answer 'yes' if the answer is even, otherwise answer 'no'.",
        }
    }
}

impl StartGame for BrainEven {
    fn task(&self) -> &str {
        self.task
    }

    fn get_correct_answer(&self, args: Vec<u8>) -> String {
        match check_evenness(args[0]) {
            true => "yes".to_string(),
            false => "no".to_string(),
        }
    }
}
