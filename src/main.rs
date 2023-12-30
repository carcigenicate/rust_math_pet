// Main Parts:
//  - Save file management
//  - Away time simulation
//  - Math/Whatever Questions
//    - A question generator trait with math subclass

// Extra Parts
//  - Shop

use rand::thread_rng;
use crate::game_state::LiveGameState;
use crate::pet::Pet;
use crate::text_ui::main_loop;

mod game_state;
mod pet;
mod text_ui;
mod text_util;
mod question_generator;

fn main() {
    let pet = Pet::new(100, 100, 100, 100);

    let mut game_state = LiveGameState {
        pet: pet,
        money: 0,
        food_per_correct: 1,
        damage_per_wrong: 1,
        starve_per_tick: 1,
        heal_per_tick: 1,
    };

    let mut random_gen = thread_rng();

    main_loop(&mut game_state, &mut random_gen);
}
