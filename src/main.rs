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
    let mut game_state = LiveGameState::load_state().unwrap_or_else(LiveGameState::new_default_state);
    let mut random_gen = thread_rng();

    let args: Vec<String> = std::env::args().collect();

    if args.get(1).unwrap_or(&"--u".to_string()) == "--u" {
        let _ = ui::egui::egui_ui::start_gui(game_state, random_gen);
    } else {
        ui::text::text_ui::main_loop(&mut game_state, &mut random_gen);
    }
}
