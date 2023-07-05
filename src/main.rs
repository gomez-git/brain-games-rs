use crate::user::User;

mod cli;
mod user;

fn main() {
    let user = User::greet();
}
