extern crate colored; // not needed in Rust 2018
use colored::*;
use std::io::Write;
#[derive(Debug)]
struct Player {
    name: String,
    health: i32,
    class: i32,
    moves: std::collections::HashMap<String, i32>,
}
#[allow(dead_code)]
struct Slime {
    health: i32,
    moves: std::collections::HashMap<String, i32>,
}
#[allow(dead_code)]
impl Slime {
    fn new() -> Slime {
        Slime {
            health: 20,
            moves: std::collections::HashMap::new(),
        }
    }
}
impl Player {
    fn new() -> Player {
        Player {
            name: String::new(),
            health: 100,
            class: 4,
            moves: std::collections::HashMap::new(),
        }
    }
}
fn main() {
    println!(
        "{}",
        "Welcome to TextLand! Where your dreams come to life!".green()
    );
    let mut player = Player::new();
    player.name = get_adventurer_name();
    println!(
        "{}{}{}",
        "Well, Hello ".green(),
        player.name.green(),
        "! Welcome to TextLand!".green()
    );
    player.class = get_type();
    match player.class {
        1 => println!("{}", "Welcome Mage!".green()),
        2 => println!("{}", "Welcome Warrior!".green()),
        3 => println!("{}", "Welcome Rogue!".green()),
        _ => println!("{}", "FATAL ERROR THIS SHOULDN'T BE POSSIBLE!".red()),
    }
    player.moves = get_moves(player.class);
    loop {
        let my_move = move_player();
        match my_move {
            1 => println!("{}{}", player.name.green(), " went North!".green()),
            2 => println!("{}{}", player.name.green(), " went South!".green()),
            3 => println!("{}{}", player.name.green(), " went East!".green()),
            4 => println!("{}{}", player.name.green(), " went West!".green()),
            _ => println!("{}", "FATAL ERROR THIS SHOULDN'T BE POSSIBLE!".red()),
        }
    }
}
fn get_adventurer_name() -> String {
    print!("{}", "What is your name adventurer?: ".cyan());
    std::io::stdout().flush().unwrap();
    let mut adventurer_name = String::new();
    std::io::stdin().read_line(&mut adventurer_name).unwrap();
    return String::from(adventurer_name.trim());
}
fn move_player() -> i32 {
    print!(
        "{}",
        "Would you like to go North(1), South(2), East(3) or West(4)?: ".cyan()
    );
    std::io::stdout().flush().unwrap();
    let mut direction = String::new();
    std::io::stdin().read_line(&mut direction).unwrap();
    let direction = direction.trim().parse();
    match direction {
        Ok(n) => {
            if n <= 0 {
                println!("{}", "Number too small!".red());
                return move_player();
            } else if n >= 5 {
                println!("{}", "Number too big!".red());
                return move_player();
            }
            return n;
        }
        Err(_n) => {
            println!("{}", "You must give a number!".cyan());
            return move_player();
        }
    }
}
fn get_type() -> i32 {
    print!(
        "{}",
        "Are you a mage(1), A warrior(2) or a rogue(3)?: ".cyan()
    );
    std::io::stdout().flush().unwrap();
    let mut adventurer_type = String::new();
    std::io::stdin().read_line(&mut adventurer_type).unwrap();
    let adventurer_type = adventurer_type.trim().parse();
    match adventurer_type {
        Ok(n) => {
            if n <= 0 {
                println!("{}", "Number too small!".red());
                return get_type();
            } else if n >= 4 {
                println!("{}", "Number too big!".red());
                return get_type();
            }
            return n;
        }
        Err(_n) => {
            println!("{}", "You must give me a number!".red());
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
    println!("{}", "Mage set works!".green());
    assert_eq!(get_moves(2), map2);
    println!("{}", "Warrior set works!".green());
    assert_eq!(get_moves(3), map3);
    println!("{}", "Rogue set works!".green());
}
