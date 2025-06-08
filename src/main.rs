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

mod game_state;
mod pet;
mod time_utils;
mod question_generator;
mod shop;
mod ui;
mod text_util;

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
    let starting_health = 100.0;
    let starting_satiation = 100.0;
    let pet = Pet::new(starting_health, starting_health, starting_satiation, starting_satiation);

    let seconds_per_day = 86400f64;
    let seconds_per_tick = 0.5;
    let ticks_per_day: f64 = seconds_per_day / seconds_per_tick;

    let tweaks = GameTweaks {
        food_per_correct: 2.5,
        damage_per_wrong: 5.0,

        ms_per_tick: (seconds_per_tick * 1000.0) as u32,

        damage_per_starved_tick: starting_health / (ticks_per_day * 2.0),
        starve_per_tick: starting_satiation / (ticks_per_day * 2.0),
        heal_per_tick: starting_health / (ticks_per_day / 2.0),
    };

    let now = time_utils::now();

    return LiveGameState {
        pet: pet,
        tweaks: tweaks,
        last_updated: now,
        created: now,
    };
}

// 1704006000000

fn main() {
    let game_state = load_state().unwrap_or_else(new_default_state);
    let random_gen = thread_rng();

    ui::egui::egui_ui::start_gui(game_state, random_gen);
}
