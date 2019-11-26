use colored::*;
use rand::Rng;
use std::io::Write;
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
        if player.health < 0 {
            println!("You are dead!");
            return;
        }
        let my_move = move_player();
        match my_move {
            1 => println!("{}{}", player.name.green(), " went North!".green()),
            2 => println!("{}{}", player.name.green(), " went South!".green()),
            3 => println!("{}{}", player.name.green(), " went East!".green()),
            4 => println!("{}{}", player.name.green(), " went West!".green()),
            _ => println!("{}", "FATAL ERROR THIS SHOULDN'T BE POSSIBLE!".red()),
        }
        let did_event_happen = monster_event();
        if did_event_happen {
            let mut is_player_turn = true;
            let mut rng = rand::thread_rng();
            let _rand_num = rng.gen_range(1, 2);
            let mut monster = Slime::new();
            println!("{}", "A Slime Appeared!".green());
            println!("Monster health = {}", monster.health);
            while monster.health > 0 {
                if player.health < 0 {
                    println!("{}", "You are dead!".red());
                    return;
                }
                if is_player_turn {
                    let attack_val = get_attack(&player);
                    let attack_power: Vec<i32> = player.moves.values().map(|i| *i as i32).collect();
                    monster.health -= attack_power[attack_val as usize];
                    println!("Monster health = {}", monster.health);
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    is_player_turn = false;
                    continue;
                } else {
                    println!("Slime tackles you dealing 3 damage");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    player.health -= 3;
                    println!("player health = {}", player.health);
                    is_player_turn = true;
                    continue;
                }
            }
            println!("You win!");
        } else {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            println!("{}", "Nothing happened!".green());
            std::thread::sleep(std::time::Duration::from_millis(1000));
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
fn get_attack(player_name: &Player) -> i32 {
    let move_iter = player_name.moves.keys();
    let mut index: i32 = 0;
    print!("{}", "Pick your move! ".cyan());
    for i in move_iter {
        print!("{}({}) ", i.green(), index);
        index += 1;
    }
    std::io::stdout().flush().unwrap();
    let mut attack_move = String::new();
    std::io::stdin().read_line(&mut attack_move).unwrap();
    let attack_move = attack_move.trim().parse();
    match attack_move {
        Ok(n) => {
            if n < 0 {
                println!("{}", "Number too small!".red());
                return get_attack(player_name);
            } else if n >= player_name.moves.len() as i32 {
                println!("{}", "Number too big!".red());
                return get_attack(player_name);
            }
            return n;
        }
        Err(_n) => {
            println!("{}", "You must give a number!".red());
            return get_attack(player_name);
        }
    }
}
fn move_player() -> i32 {
    print!(
        "{} {}(1) {}(2) {}(3) or {}(4) ",
        "Would you like to go".cyan(),
        "North".green(),
        "South".green(),
        "East".green(),
        "West".green()
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
            println!("{}", "You must give a number!".red());
            return move_player();
        }
    }
}
fn get_type() -> i32 {
    print!(
        "{} {}(1) {}(2) or a {}(3) ",
        "Are you a".cyan(),
        "Mage".green(),
        "A Warrior".green(),
        "Rogue".green()
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
        map.insert(String::from("Fireball"), 4);
    // map.insert(String::from("Heal"), 5);
    // map.insert(String::from("Ward"), 14);
    } else if class == 2 {
        map.insert(String::from("Sword"), 2);
        // map.insert(String::from("Sheild"), 7);
        map.insert(String::from("Hammer"), 5);
    } else if class == 3 {
        map.insert(String::from("Dagger"), 1);
        // map.insert(String::from("Invisibility"), 20);
        map.insert(String::from("Bow"), 6);
    }
    return map;
}
fn monster_event() -> bool {
    let mut rng = rand::thread_rng();
    let rand_num = rng.gen::<i32>();
    if rand_num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
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
            health: 70,
            class: 4,
            moves: std::collections::HashMap::new(),
        }
    }
}
