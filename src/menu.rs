pub struct Menu {
    buttons: [&'static str; 5],
}

impl Menu {
    pub fn new() -> Self {
        Self {
            buttons: [
                "[f] Brain Even",
                "[d] Brain Dice",
                "[k] Brain Calculator",
                "[s] Brain Greatest Common Divisor",
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
