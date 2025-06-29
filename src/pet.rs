use std::cmp::{max, min};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Pet {
    pub health: f64,
    pub health_max: f64,

    pub satiation: f64,
    pub satiation_max: f64,

    pub level: u128,
    pub experience: f64,
    pub next_level_at: f64,
}

impl Pet {
    pub fn new(health: f64, health_max: f64, satiation: f64, satiation_max: f64) -> Self {
        Pet {
            health: health,
            health_max: health_max,
            satiation: satiation,
            satiation_max: satiation_max,
            level: 1,
            experience: 0.0,
            next_level_at: 2.0,
        }
    }

    pub fn feed(&mut self, food_value: f64) -> () {
        self.satiation = (self.satiation + food_value).clamp(0.0, self.satiation_max);
    }

    pub fn starve(&mut self, starve_value: f64) -> () {
        self.satiation = (self.satiation - starve_value).clamp(0.0, self.satiation_max);
    }

    pub fn heal(&mut self, health_value: f64) -> () {
        self.health = (self.health + health_value).clamp(0.0, self.health_max);
    }

    pub fn hurt(&mut self, damage_value: f64) -> () {
        self.health = (self.health - damage_value).clamp(0.0, self.health_max);
    }

    pub fn give_experience(&mut self, experience: f64) -> () {
        self.experience += experience;

        if self.should_level() {
            self.level_up();
        }
    }

    pub fn remove_experience(&mut self, experience: f64) -> () {
        let new_experience = self.experience - experience;
        self.experience = new_experience.max(0.0);
    }

    pub fn should_level(&self) -> bool {
        self.experience >= self.next_level_at
    }

    pub fn level_up(&mut self) {
        // TODO: Check if we should level first?
        let overshot_exp = self.experience - self.next_level_at;

        self.level += 1;
        self.experience = overshot_exp.max(0.0);
        self.next_level_at *= 1.1;  // TODO: Should these magic numbers be saved in the file, or as constants in the code?

        self.health_max += 1.0;
        self.satiation_max += 1.0;
    }

    pub fn has_full_health(&self) -> bool {
        self.satiation >= self.satiation_max
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    pub fn is_full(&self) -> bool {
        self.satiation >= self.satiation_max
    }

    pub fn is_starving(&self) -> bool {
        self.satiation <= 0.0
    }

    pub fn format_stats(&self) -> String {
        format!("Health: {:.1}/{:.1}, Satiation: {:.1}/{:.1}", self.health, self.health_max, self.satiation, self.satiation_max)
    }
}