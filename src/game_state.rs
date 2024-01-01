use serde::{Deserialize, Serialize};
use crate::pet::Pet;
use crate::time_utils;

#[derive(Serialize, Deserialize)]
pub struct GameTweaks {
    pub food_per_correct: f64,
    pub damage_per_wrong: f64,
    pub damage_per_starved_tick: f64,
    pub starve_per_tick: f64,
    pub heal_per_tick: f64,

    pub ms_per_tick: u32,
}

#[derive(Serialize, Deserialize)]
pub struct LiveGameState {
    pub pet: Pet,
    pub money: u32,
    pub last_updated: u128,

    pub tweaks: GameTweaks,
    // question_generator: ?
}

impl LiveGameState {
    // fn new(pet: Pet, starting_money: u32) -> Self {
    //     return Self {
    //         pet: pet,
    //         money: starting_money,
    //     };
    // }

    pub fn borrow_pet(&mut self) -> &mut Pet {
        return &mut self.pet;
    }

    pub fn damage_pet(&mut self) {
        self.pet.hurt(self.tweaks.damage_per_wrong);
    }

    pub fn feed_pet(&mut self) {
        self.pet.feed(self.tweaks.food_per_correct);
    }

    pub fn advance_tick(&mut self) {
        self.pet.starve(self.tweaks.starve_per_tick);

        if self.pet.is_starving() {
            self.pet.hurt(self.tweaks.damage_per_starved_tick);
        } else {
            self.pet.heal(self.tweaks.heal_per_tick);
        }
    }

    pub fn account_for_elapsed_time(&mut self) {
        let now = time_utils::now();
        let ms_elapsed = now - self.last_updated;
        let ticks_elapsed = ms_elapsed / self.tweaks.ms_per_tick as u128;

        println!("Elapsed: {:.2} hours ({} ticks)", ms_elapsed as f64 / 1000.0 / 60.0 / 60.0, ticks_elapsed);

        for _ in 0..ticks_elapsed {
            self.advance_tick();

            if self.is_game_over() {
                break;
            }
        }

        self.last_updated = now;
    }

    pub fn is_game_over(&self) -> bool {
        return self.pet.is_dead();
    }

    pub fn format_stats(&self) -> String {
        return format!("Money: {}, Pet: {}", self.money, self.pet.format_stats());
    }
}