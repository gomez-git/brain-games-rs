pub struct Menu {
    buttons: [&'static str; 7],
}

impl Menu {
    pub fn new() -> Self {
        Self {
            buttons: [
                "[f] Brain Even",
                "[j] Brain Prime",
                "[d] Brain Dice",
                "[k] Brain Calculator",
                "[s] Brain Greatest Common Divisor",
                "[l] Brain Progression",
                "[\u{23CE}] Exit",
            ],
        }
    }

    pub fn print(&self) {
        for button in self.buttons {
            println!("{button}");
        }
    }
}
