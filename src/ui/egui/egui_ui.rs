use std::time::Duration;
use eframe;
use eframe::{egui, CreationContext};
use eframe::egui::{Align, Event, FontFamily, InputState};
use egui::{FontId, Layout};
use crate::game_state::LiveGameState;
use rand::{thread_rng};
use rand::rngs::ThreadRng;
use crate::{new_default_state, save_state};
use crate::shop;
use crate::question_generator::math_question_generator;

const INITIAL_WINDOW_SIZE: (f32, f32) = (640.0, 300.0);

pub fn start_gui(game: LiveGameState, rand_gen: ThreadRng) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([INITIAL_WINDOW_SIZE.0 as f32, INITIAL_WINDOW_SIZE.1 as f32]),
        ..Default::default()
    };

    eframe::run_native(
        "Math Pet",
        options,
        Box::new(|cc| {
            let state = UiState::new(game, rand_gen);

            let font_id = FontId::new(16.0, FontFamily::Proportional);
            let mut style = (*cc.egui_ctx.style()).clone();
            style.override_font_id = Some(font_id);
            cc.egui_ctx.set_style(style);

            egui_extras::install_image_loaders(&cc.egui_ctx);
            return Ok(Box::<UiState>::new(state));
        }),
    )
}

fn handle_closing(game_state: &mut LiveGameState) {
    // if game_state.is_game_over() {
    //     println!("Your pet died! Restarting...");
    //     game_state = new_default_state()
    // } else {
    //
    // }

    game_state.account_for_elapsed_time();
    save_state(&game_state);
    println!("Saved...");
}

struct UiState {
    game: LiveGameState,
    shop_items: Vec<shop::ShopItem>,
    rand_gen: ThreadRng,

    // ui state
    math_input_text_buffer: String,
    current_question: String,
    current_question_answer: i32,
    question_history: Vec<String>,
    status: String,
}

impl UiState {
    fn new(game: LiveGameState, mut rand_gen: ThreadRng) -> Self {
        let shop_items = shop::get_shop_inventory();

        let (question, answer) = math_question_generator::generate(&mut rand_gen);

        Self {
            game,
            rand_gen,
            math_input_text_buffer: String::new(),
            shop_items,
            current_question: question,
            current_question_answer: answer,
            question_history: Vec::new(),
            status: String::new(),
        }
    }

    fn evaluate_question(&mut self) {
        match self.math_input_text_buffer.parse::<i32>() {
            Ok(user_answer) => {
                let status = if user_answer == self.current_question_answer {
                    self.game.pet.feed(self.game.tweaks.food_per_correct);
                    self.set_status("Correct");
                    String::from("Correct")
                } else {
                    self.game.pet.hurt(self.game.tweaks.damage_per_wrong);
                    self.set_status("Incorrect");
                    format!("Wrong ({})", self.current_question_answer)
                };

                let history_str = format!("{} = {}    {}", self.current_question, user_answer, status);

                let (question, answer) = math_question_generator::generate(&mut self.rand_gen);
                self.current_question = question;
                self.current_question_answer = answer;
                self.math_input_text_buffer = String::new();

                self.question_history.push(history_str);
            },
            _ => {}
        }

    }

    fn set_status<S: Into<String>>(&mut self, status: S) {
        self.status = status.into();
    }

    fn check_for_and_handle_death(&mut self) {
        if self.game.is_game_over() {
            self.set_status("Your Pet Died. Resetting...");

            self.game = new_default_state();
            save_state(&self.game);
        }
    }
}

fn apply_event(event: &Event, input_state: &InputState) {

}

impl eframe::App for UiState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(| i | {
            for event in &i.events {
                apply_event(&event, i);
            }

            if i.viewport().close_requested() {
                handle_closing(&mut self.game);
            }
        });

        egui::TopBottomPanel::top("status_panel").show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.label(self.status.clone());
            });
        });

        egui::SidePanel::left("stats_panel").show(ctx, |ui| {
            ui.label(format!("Health: {:.1}/{:.1}", self.game.pet.health, self.game.pet.health_max));
            ui.label(format!("Satiation: {:.1}/{:.1}", self.game.pet.satiation, self.game.pet.satiation_max));
        });

        egui::SidePanel::right("shop_panel").show(ctx, |ui| {
            for item in &self.shop_items {
                ui.horizontal(|ui| {
                    let btn = ui.button("Buy");
                    ui.label(format!("{}: {} sat", item.name, item.price));

                    if btn.clicked() {
                        item.buy_and_apply(&mut self.game);
                    }
                });
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.horizontal(|ui| {
                    let label = ui.label(format!("{} = ", self.current_question));
                    let text_edit = ui.text_edit_singleline(&mut self.math_input_text_buffer);

                    if text_edit.lost_focus() && text_edit.ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.evaluate_question();
                        text_edit.request_focus();
                    }
                });

                for history_entry in self.question_history.iter().rev() {
                    ui.label(history_entry.clone());
                }
            });
        });

        self.game.account_for_elapsed_time();
        self.check_for_and_handle_death();

        ctx.request_repaint_after(Duration::from_secs(1));
    }
}