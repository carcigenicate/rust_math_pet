// Main Parts:
//  - Save file management
//  - Away time simulation
//  - Math/Whatever Questions
//    - A question generator trait with math subclass

// Extra Parts
//  - Shop

use rand::thread_rng;
use crate::game_state::{GameStats, GameTweaks, LiveGameState};
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

// 1704006000000

fn main() {
    let game_state = load_state().unwrap_or_else(LiveGameState::new_default_state);
    let random_gen = thread_rng();

    ui::egui::egui_ui::start_gui(game_state, random_gen);
}
