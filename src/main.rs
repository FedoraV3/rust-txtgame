use enemy::NewEnemy;
use player::Actions;

mod enemy;
mod player;

fn Wave(x: i128, y: &str) -> Vec<NewEnemy> {
    let mut _New_Wave: Vec<NewEnemy> = Vec::new();

    for _i in 1..x {
        _New_Wave.push(enemy::create_enemy(y));
    }

    _New_Wave
}

fn main() {
    let player = player::Player {
        name: String::from("Player"),
        health: 100,
        resistance: 10,
        damage: 20,
        weapon: String::from("Normal"),
    };

    println!("Hello, {}. Your starting stats are: HP: {} Resistance: {}, DMG: {}, Weapon: {}", player.name, player.health, player.resistance, player.damage, player.weapon);


    let mut enemies = Wave(10, "Normal");

    for i in enemies {
        println!("{}", i.health);
    }
}