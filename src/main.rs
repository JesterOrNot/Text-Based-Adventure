#[macro_use]
extern crate colour;

use std::io::Write;

fn main() {
    green!("Welcome to TextLand! Where your dreams come to life!\n");
    let name = get_adventurer_name();
    green!("Well, Hello {}! Welcome to TextLand\n", name)
}
fn get_adventurer_name() -> String {
    cyan!("What is your name adventurer?: ");
    std::io::stdout().flush().unwrap();
    let mut adventurer_name = String::new();
    std::io::stdin().read_line(&mut adventurer_name).unwrap();
    return String::from(adventurer_name.trim());
}
// fn get_type() {
//     print
// }
