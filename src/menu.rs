pub struct Menu {
    buttons: [&'static str; 3],
}

impl Menu {
    pub fn new() -> Self {
        Self {
            buttons: [
                "[f] Brain Even",
                "[d] Brain Dice",
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
