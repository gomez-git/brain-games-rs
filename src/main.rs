use crate::menu::Menu;
use crate::user::User;

mod cli;
mod games;
mod math;
mod menu;
mod user;

fn main() {
    let user = User::greet();

    let menu = Menu::new();

    loop {
        menu.print();

        let game = user.choose_game();

        let result = game.run();

        user.react_on_result(result);
    }
}
