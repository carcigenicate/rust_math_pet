use rand::rngs::ThreadRng;
use serde::{Deserialize, Serialize};
use crate::pet::Pet;

pub struct LiveGameState {
    pub pet: Pet,
    pub money: u32,

    pub food_per_correct: u32,
    pub damage_per_wrong: u32,
    pub starve_per_tick: u32,
    pub heal_per_tick: u32,

    // question_generator: ?
}

impl LiveGameState {
    // fn new(pet: Pet, starting_money: u32) -> Self {
    //     return Self {
    //         pet: pet,
    //         money: starting_money,
    //     };
    // }

    pub fn borrow_pet(&mut self) -> &mut Pet {
        return &mut self.pet;
    }
}