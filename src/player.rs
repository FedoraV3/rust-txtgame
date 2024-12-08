use rand::{thread_rng, Rng};

use crate::enemy;

pub struct Player {
    pub name: String,
    pub health: i128,
    pub damage: i128,
    pub resistance: i128,
    pub weapon: String,
}

pub trait Actions {
    fn attk(&self, enemy: &mut enemy::NewEnemy) -> i128;
    fn heal(&mut self);
}

impl Actions for Player {
    fn attk(&self, enemy: &mut enemy::NewEnemy) -> i128 {
        enemy.health -= self.damage - enemy.resistance;

        self.damage
    }

    fn heal(&mut self) {
        let mut tr = thread_rng();
        self.health += tr.gen_range(1..30);
    }
}