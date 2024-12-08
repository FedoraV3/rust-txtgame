use rand::{thread_rng, Rng};

pub struct EnemyWeapon(String);

impl EnemyWeapon { 
    pub fn new(name: &str) -> Self {
        EnemyWeapon(name.to_string())
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

pub struct NewEnemy {
    pub health: i128,
    pub damage: i128,
    pub resistance: i128,
    pub weapon: EnemyWeapon,
}


pub fn create_enemy(enemytype: &str) -> NewEnemy {
    let mut rng: rand::prelude::ThreadRng = thread_rng();

    let trgt = if enemytype == "Normal" {
        NewEnemy {
            health: rng.gen_range(30..100),
            damage: rng.gen_range(20..50),
            resistance: rng.gen_range(1..10),
            weapon: EnemyWeapon::new("Pistol"),
        }
    } else if enemytype == "Assault" {
        NewEnemy {
            health: rng.gen_range(50..100),
            damage: rng.gen_range(15..45),
            resistance: rng.gen_range(5..15),
            weapon: EnemyWeapon::new("Rifle"),
        }
    } else {
        panic!("Unknown enemy type: {}", enemytype);
    };

    trgt
}


