#[macro_use]
extern crate colour;
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
    green!("Welcome to TextLand! Where your dreams come to life!\n");
    let mut player = Player::new();
    player.name = get_adventurer_name();
    green!("Well, Hello {}! Welcome to TextLand\n", player.name);
    player.class = get_type();
    player.moves = get_moves(player.class);
    println!("{:?}", player);
    loop {
        let myMove = movePlayer();
        println!("{}",myMove);
    }
}
fn get_adventurer_name() -> String {
    cyan!("What is your name adventurer?: ");
    std::io::stdout().flush().unwrap();
    let mut adventurer_name = String::new();
    std::io::stdin().read_line(&mut adventurer_name).unwrap();
    return String::from(adventurer_name.trim());
}
fn movePlayer() -> i32 {
    cyan!("Would you like to go North(1), South(2), East(3) or West(4)?: ");
    std::io::stdout().flush().unwrap();
    let mut direction = String::new();
    std::io::stdin().read_line(&mut direction).unwrap();
    let direction = direction.trim().parse();
    match direction {
        Ok(n) => n,
        Err(_n) => {
            println!("You must give me a number!");
            return movePlayer();
        }
    }
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
