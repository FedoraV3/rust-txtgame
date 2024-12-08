mod enemy;
mod player;
mod game_handler;

use enemy::NewEnemy;
use game_handler::{cast_spell, update, wave};
use rand::{thread_rng, Rng};
use std::io;
use player::Actions;

fn main() {
    let mut tr = thread_rng();
    let mut player = player::Player {
        name: String::from("Player"),
        health: 100,
        resistance: 10,
        damage: 20,
        weapon: String::from("Normal"),
    };

    println!("Hello, {}. Your starting stats are: HP: {} Resistance: {}, DMG: {}, Weapon: {}", player.name, player.health, player.resistance, player.damage, player.weapon);
    println!("Starting new wave...");

    let enemiestospawn = tr.gen_range(1..10);

    let mut new_wave = wave(enemiestospawn, "Normal");

    println!("Wave has started!");

    loop {
        if player.health <= 0 { println!("You have died. 'cargo run' to Restart!"); break; }

        println!("A: Attk H: Heal (get 40hp)");
        let mut val: String = String::new();
        io::stdin()
            .read_line(&mut val)
            .expect("Failed to read line; did you try something stupid?");

        
        if val.trim() == "A" {
            println!("Attack which enemy? There are {} enemies", enemiestospawn - 1);
            let mut response = String::new();
            io::stdin()
                .read_line(&mut response)
                .expect("Failed to read line; did you do something stupid?");
            
            let parsedint: usize = response.trim().parse().unwrap();
            

            println!("Original Enemy HP: {}", new_wave[parsedint].health);
            player.attk(&mut new_wave[parsedint]);
            println!("After enemy took damage: {}, Resisted: {} of HP", new_wave[parsedint].health, new_wave[parsedint].resistance);

            let chanceofcounter = tr.gen_range(1..2);

            if chanceofcounter == 1 {
                player.health -= new_wave[parsedint].damage - player.resistance;
                println!("Oof enemy got a chance to hit you, but you resisted {} dmg", player.resistance);
            } else {
                continue;
            }
            
        } else if val.trim() == "H" {
            player.heal();
        } else if val.trim() == "S" {
            println!("Using AOE!");

            cast_spell("AOE", &mut new_wave);
        }

        update(&mut player, &mut new_wave);
    }
}