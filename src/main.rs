// Main Parts:
//  - Save file management
//  - Away time simulation
//  - Math/Whatever Questions
//    - A question generator trait with math subclass

// Extra Parts
//  - Shop

use rand::thread_rng;
use crate::game_state::{GameTweaks, LiveGameState};
use crate::pet::Pet;
use crate::text_ui::main_loop;

mod game_state;
mod pet;
mod text_ui;
mod text_util;
mod time_utils;
mod question_generator;
mod shop;

const SAVE_PATH: &str = "./pet_save.json";

fn load_state() -> Option<LiveGameState> {
    return std::fs::read_to_string(SAVE_PATH).map_or(None, |deserialized| {
        return serde_json::from_str(deserialized.as_str()).unwrap_or(None);
    });
}

fn save_state(game_state: &LiveGameState) -> () {
    match serde_json::to_string(game_state) {
        Ok(serialized) => std::fs::write(SAVE_PATH, serialized).expect("Writing save file"),
        Err(err) => {
            eprintln!("Saving failed with error {err}");
        },
    }
}

fn new_default_state() -> LiveGameState {
    let pet = Pet::new(100.0, 100.0, 100.0, 100.0);

    let tweaks = GameTweaks {
        food_per_correct: 2.0,
        damage_per_wrong: 2.0,

        ms_per_tick: 500,

        damage_per_starved_tick: 5e-4,
        starve_per_tick: 5e-4,
        heal_per_tick: 5e-4,
    };

    return LiveGameState {
        pet: pet,
        money: 0,
        tweaks: tweaks,
        last_updated: time_utils::now(),
    };
}

// 1704006000000

fn main() {
    let mut game_state = load_state().unwrap_or_else(new_default_state);
    let mut random_gen = thread_rng();

    main_loop(&mut game_state, &mut random_gen);

    if game_state.is_game_over() {
        println!("Your pet died! Restarting...");
        game_state = new_default_state()
    } else {
        game_state.account_for_elapsed_time();
    }

    save_state(&game_state);
}
