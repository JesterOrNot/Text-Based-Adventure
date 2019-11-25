use std::io::Write;
fn main() {
    let name = get_adventurer_name();
    println!("Well Hello {}!", name)
}
fn get_adventurer_name() -> String {
    print!("What is your name adventurer?: ");
    std::io::stdout().flush().unwrap();
    let mut adventurer_name = String::new();
    std::io::stdin().read_line(&mut adventurer_name).unwrap();
    return String::from(adventurer_name.trim());
}
