use std::cmp::{max, min};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Pet {
    health: u32,
    health_max: u32,

    satiation: u32,
    satiation_max: u32,

    // birth_date: ?
}

impl Pet {
    pub fn new(health: u32, health_max: u32, satiation: u32, satiation_max: u32) -> Self {
        return Pet {
            health: health,
            health_max: health_max,
            satiation: satiation,
            satiation_max: satiation_max,
        };
    }

    pub fn feed(&mut self, food_value: u32) -> () {
        self.satiation = self.satiation
            .checked_add(food_value)
            .map_or(self.satiation_max, |new_sat| min(new_sat, self.satiation_max));
    }

    pub fn starve(&mut self, starve_value: u32) -> () {
        self.satiation = self.satiation
            .checked_sub(starve_value)
            .unwrap_or(0);
    }

    pub fn heal(&mut self, health_value: u32) -> () {
        self.health = self.health
            .checked_add(health_value)
            .map_or(self.health_max, |new_health| min(new_health, self.health_max));
    }

    pub fn hurt(&mut self, damage_value: u32) -> () {
        self.health = self.health
            .checked_sub(damage_value)
            .unwrap_or(0);
    }

    pub fn has_full_health(&self) -> bool {
        return self.satiation == self.satiation_max;
    }

    pub fn is_dead(&self) -> bool {
        return self.health == 0;
    }

    pub fn is_full(&self) -> bool {
        return self.satiation == self.satiation_max;
    }

    pub fn is_starving(&self) -> bool {
        return self.satiation == 0;
    }

    pub fn format_stats(&self) -> String {
        return format!("Health: {}/{}, Satiation: {}/{}", self.health, self.health_max, self.satiation, self.satiation_max);
    }
}