#[macro_use]
extern crate colour;

use std::io::Write;

fn main() {
    green!("Welcome to TextLand! Where your dreams come to life!\n");
    let name = get_adventurer_name();
    green!("Well, Hello {}! Welcome to TextLand\n", name);
    let class = get_type();
}
fn get_adventurer_name() -> String {
    cyan!("What is your name adventurer?: ");
    std::io::stdout().flush().unwrap();
    let mut adventurer_name = String::new();
    std::io::stdin().read_line(&mut adventurer_name).unwrap();
    return String::from(adventurer_name.trim());
}
fn get_type() -> Result<i32,std::num::ParseIntError> {
    cyan!("Are you a mage(1), A warrior(2) or a rogue(3)?: ");
    std::io::stdout().flush().unwrap();
    let mut adventurer_type = String::new();
    std::io::stdin().read_line(&mut adventurer_type).unwrap();
    let adventurer_type = adventurer_type.trim().parse();
    match adventurer_type {
        Ok(_n) => adventurer_type,
        Err(_n) => {
            println!("You must give me a number!");
            return get_type();
        }
    }
}
