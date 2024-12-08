use crate::NewEnemy;
use crate::enemy;
use crate::player::Player;
use rand::{thread_rng, Rng};


pub fn wave(x: i128, y: &str) -> Vec<NewEnemy> {
    let mut _new_wave: Vec<NewEnemy> = Vec::new();

    for _i in 1..x {
        _new_wave.push(enemy::create_enemy(y));
    }

    _new_wave
}

pub fn cast_spell(spell: &str, enemyvect: &mut Vec<NewEnemy>) {
    let mut rng = thread_rng();

    if spell == "AOE" {
        let dmg_amount = rng.gen_range(1..30);

        for enemy in enemyvect {
            enemy.health -= dmg_amount - enemy.resistance;
        }
    }
}

pub fn update<'a>(player: &mut Player, current_wave: &mut Vec<NewEnemy>) {
    let mut dmg_pool = 0; 
    for dmg_gotten in &mut *current_wave {
        dmg_pool += dmg_gotten.damage;
    }

    player.health -= dmg_pool;

    for enemy in current_wave {
        if enemy.health == 0 {
            println!("You killed an enemy! His weapon was: {}", enemy.weapon.name());
        }
    }
}