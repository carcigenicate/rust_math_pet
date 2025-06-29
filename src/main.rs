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

fn main() {
    let game_state = LiveGameState::load_state().unwrap_or_else(LiveGameState::new_default_state);
    let random_gen = thread_rng();

    let _ = ui::egui::egui_ui::start_gui(game_state, random_gen);
}
