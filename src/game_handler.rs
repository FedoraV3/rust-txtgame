use crate::NewEnemy;
use crate::enemy;

pub fn wave(x: i128, y: &str) -> Vec<NewEnemy> {
    let mut _new_wave: Vec<NewEnemy> = Vec::new();

    for _i in 1..x {
        _new_wave.push(enemy::create_enemy(y));
    }

    _new_wave
}