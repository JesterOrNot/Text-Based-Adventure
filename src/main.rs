#[macro_use]
extern crate colour;

use std::io::Write;

fn main() {
    green!("Welcome to TextLand! Where your dreams come to life!\n");
    let name = get_adventurer_name();
    green!("Well, Hello {}! Welcome to TextLand\n", name);
    let class = get_type();
    let move_list = get_moves(class);
    println!("{:?}", move_list);
}
fn get_adventurer_name() -> String {
    cyan!("What is your name adventurer?: ");
    std::io::stdout().flush().unwrap();
    let mut adventurer_name = String::new();
    std::io::stdin().read_line(&mut adventurer_name).unwrap();
    return String::from(adventurer_name.trim());
}
fn get_type() -> i32 {
    cyan!("Are you a mage(1), A warrior(2) or a rogue(3)?: ");
    std::io::stdout().flush().unwrap();
    let mut adventurer_type = String::new();
    std::io::stdin().read_line(&mut adventurer_type).unwrap();
    let adventurer_type = adventurer_type.trim().parse();
    match adventurer_type {
        Ok(n) => n,
        Err(_n) => {
            println!("You must give me a number!");
            return get_type();
        }
    }
}
fn get_moves(class: i32) -> std::collections::HashMap<String, i32> {
    let mut map = std::collections::HashMap::new();
    if class == 1 {
        map.insert(String::from("Fireball"), 10);
        map.insert(String::from("Heal"), 5);
        map.insert(String::from("Ward"), 14);
    } else if class == 2 {
        map.insert(String::from("Sword"), 12);
        map.insert(String::from("Sheild"), 7);
        map.insert(String::from("Hammer"), 20);
    } else if class == 3 {
        map.insert(String::from("Dagger"), 7);
        map.insert(String::from("Invisibility"), 20);
        map.insert(String::from("Bow"), 5);
    }
    return map;
}
#[test]
fn test_get_moves() {
    let mut map = std::collections::HashMap::new();
    let mut map2 = std::collections::HashMap::new();
    let mut map3 = std::collections::HashMap::new();
    map.insert(String::from("Fireball"), 10);
    map.insert(String::from("Heal"), 5);
    map.insert(String::from("Ward"), 14);
    map2.insert(String::from("Sword"), 12);
    map2.insert(String::from("Sheild"), 7);
    map2.insert(String::from("Hammer"), 20);
    map3.insert(String::from("Dagger"), 7);
    map3.insert(String::from("Invisibility"), 20);
    map3.insert(String::from("Bow"), 5);
    assert_eq!(get_moves(1), map);
    green!("Mage set works!\n");
    assert_eq!(get_moves(2), map2);
    green!("Warrior set works!\n");
    assert_eq!(get_moves(3), map3);
    green!("Rogue set works!\n");
}
