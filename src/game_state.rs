use std::cmp::min;
use std::mem;
use serde::{Deserialize, Serialize};
use crate::pet::Pet;
use crate::time_utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameTweaks {
    pub food_per_correct: f64,
    pub damage_per_wrong: f64,
    pub damage_per_starved_tick: f64,
    pub starve_per_tick: f64,
    pub heal_per_tick: f64,

    pub ms_per_tick: u32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameStats {
    pub answered_correct: u128,
    pub answered_incorrect: u128,
    pub damage_taken: f64,
    pub damage_healed: f64,
    pub amount_fed: f64,
}

impl GameStats {
    pub fn new() -> GameStats {
        Self {
            answered_correct: 0,
            answered_incorrect: 0,
            amount_fed: 0.0,
            damage_healed: 0.0,
            damage_taken: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HistoryState {
    pub pet: Pet,
    pub tweaks: GameTweaks,
    pub stats: GameStats,
    pub created: u128,
    pub ended: u128,
}

impl HistoryState {
    pub fn from_game_state(alive_state: &LiveGameState) -> Self {
        Self {
            pet: alive_state.pet.clone(),
            created: alive_state.created,
            stats: alive_state.stats.clone(),
            tweaks: alive_state.tweaks.clone(),
            ended: time_utils::now_as_milli(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LiveGameState {
    pub pet: Pet,
    pub tweaks: GameTweaks,

    pub stats: GameStats,

    // Timestamps
    pub last_updated: u128,
    pub created: u128,

    pub game_history: Vec<HistoryState>,
}

impl LiveGameState {
    pub fn new_default_state() -> Self {
        let starting_health = 100.0;
        let starting_satiation = 100.0;
        let pet = Pet::new(starting_health, starting_health, starting_satiation, starting_satiation * 2.0);

        let seconds_per_day = 86400f64;
        let seconds_per_tick = 1.0;
        let ticks_per_day: f64 = seconds_per_day / seconds_per_tick;

        let tweaks = GameTweaks {
            food_per_correct: 2.5,
            damage_per_wrong: 5.0,

            ms_per_tick: (seconds_per_tick * 1000.0) as u32,

            damage_per_starved_tick: starting_health / (ticks_per_day / 2.0),
            starve_per_tick: starting_satiation / (ticks_per_day / 2.0),
            heal_per_tick: starting_health / (ticks_per_day / 4.0),
        };

        let now = time_utils::now_as_milli();

        Self {
            pet: pet,
            tweaks: tweaks,
            stats: GameStats::new(),
            last_updated: now,
            created: now,
            game_history: vec![]
        }
    }

    pub fn ticks_to_starving_and_death(&self) -> (u128, u128) {
        let mut copy_self = self.clone();  // Eww  // FIXME:

        let mut starving = 0;
        let mut ticks = 0;
        while !copy_self.pet.is_dead() {
            copy_self.advance_tick();
            ticks += 1;

            if starving == 0 && copy_self.pet.is_starving() {
                starving = ticks;
            }
        }

        (starving, ticks)
    }

    pub fn hours_to_starving_and_death(&self) -> (f64, f64) {
        let (ticks_until_starving, ticks_until_death) = self.ticks_to_starving_and_death();

        let ms_until_starving = (ticks_until_starving * (self.tweaks.ms_per_tick as u128)) as f64;
        let ms_until_death = (ticks_until_death * (self.tweaks.ms_per_tick as u128)) as f64;

        (
            ms_until_starving / 1000.0 / 60.0 / 60.0,
            ms_until_death / 1000.0 / 60.0 / 60.0,
        )
    }

    pub fn borrow_pet(&mut self) -> &mut Pet {
        &mut self.pet
    }

    pub fn damage_pet(&mut self, by: f64) {
        if !self.pet.is_dead() {
            self.pet.hurt(by);
            self.stats.damage_taken += by;
        }
    }

    pub fn feed_pet(&mut self) {
        if !self.pet.is_full() {
            self.pet.feed(self.tweaks.food_per_correct);
            self.stats.amount_fed += self.tweaks.food_per_correct.min(self.pet.satiation_max - self.pet.satiation);
        }
    }

    pub fn heal_pet(&mut self, by: f64) {
        if !self.pet.has_full_health() {
            self.pet.heal(by);
            self.stats.damage_healed += by.min(self.pet.health_max - self.pet.health);
        }
    }

    pub fn advance_tick(&mut self) {
        self.pet.starve(self.tweaks.starve_per_tick);

        if self.pet.is_starving() {
            self.damage_pet(self.tweaks.damage_per_starved_tick);
        } else {
            self.heal_pet(self.tweaks.heal_per_tick);
        }
    }

    pub fn record_question(&mut self, answered_correct: bool) {
        if answered_correct {
            self.stats.answered_correct += 1;
        } else {
            self.stats.answered_incorrect += 1;
        }
    }

    pub fn account_for_elapsed_time(&mut self) {
        let now = time_utils::now_as_milli();
        let ms_elapsed = now - self.last_updated;
        let ticks_elapsed = ms_elapsed / self.tweaks.ms_per_tick as u128;

        if ticks_elapsed > 0 {
            for _ in 0..ticks_elapsed {
                self.advance_tick();

                if self.is_game_over() {
                    break;
                }
            }

            self.last_updated = now;
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.pet.is_dead()
    }

    pub fn format_stats(&self) -> String {
        self.pet.format_stats()
    }

    pub fn record_and_reset(&mut self) {
        let history_entry = HistoryState::from_game_state(self);
        self.game_history.push(history_entry);

        let history = self.game_history.to_vec();

        let _ = mem::replace(self, LiveGameState::new_default_state());
        self.game_history = history;
    }
}