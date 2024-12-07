use enemy::NewEnemy;
use game_handler::wave;

mod enemy;
mod player;
mod game_handler;

fn main() {
    let player = player::Player {
        name: String::from("Player"),
        health: 100,
        resistance: 10,
        damage: 20,
        weapon: String::from("Normal"),
    };

    println!("Hello, {}. Your starting stats are: HP: {} Resistance: {}, DMG: {}, Weapon: {}", player.name, player.health, player.resistance, player.damage, player.weapon);


    let mut enemies = wave(10, "Normal");

    for i in enemies {
        println!("{}", i.health);
    }
}