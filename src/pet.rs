use std::cmp::{max, min};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pet {
    pub health: f64,
    pub health_max: f64,

    pub satiation: f64,
    pub satiation_max: f64,

    // birth_date: ?
}

impl Pet {
    pub fn new(health: f64, health_max: f64, satiation: f64, satiation_max: f64) -> Self {
        return Pet {
            health: health,
            health_max: health_max,
            satiation: satiation,
            satiation_max: satiation_max,
        };
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

    pub fn has_full_health(&self) -> bool {
        return self.satiation == self.satiation_max;
    }

    pub fn is_dead(&self) -> bool {
        return self.health == 0.0;
    }

    pub fn is_full(&self) -> bool {
        return self.satiation == self.satiation_max;
    }

    pub fn is_starving(&self) -> bool {
        return self.satiation == 0.0;
    }

    pub fn format_stats(&self) -> String {
        return format!("Health: {:.1}/{:.1}, Satiation: {:.1}/{:.1}", self.health, self.health_max, self.satiation, self.satiation_max);
    }
}