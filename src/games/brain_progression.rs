use super::super::math::find_missing_number_in_progression;
use super::StartGame;

pub struct BrainProgression {
    task: &'static str,
}

impl BrainProgression {
    pub fn new() -> Self {
        Self {
            task: "What number is missing in the progression?",
        }
    }

    fn get_progression(&self, first_number: u8, step: u8, quantity: u8) -> Vec<u8> {
        let mut progression = vec![first_number];
        let mut i: u8 = 1;

        while i < quantity {
            progression.push(first_number + step * i);
            i += 1;
        }

        progression
    }
}

impl StartGame for BrainProgression {
    fn task(&self) -> &str {
        self.task
    }

    fn get_args(&self) -> Vec<u8> {
        let first_number = self.get_random_number(1, 100);
        let step = self.get_random_number(5, 20);
        let quantity = self.get_random_number(5, 10);

        let mut progression = self.get_progression(first_number, step, quantity);

        let index = self.get_random_number(0, quantity - 1) as usize;

        progression[index] = 0;

        progression
    }

    fn question(&self, args: &Vec<u8>) -> String {
        args.iter()
            .map(|&num| match num {
                0 => "..".to_string(),
                _ => num.to_string(),
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn get_correct_answer(&self, args: Vec<u8>) -> String {
        find_missing_number_in_progression(args).to_string()
    }
}
