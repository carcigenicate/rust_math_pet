use rand::rngs::ThreadRng;
use crate::game_state::LiveGameState;
use crate::text_util::{get_input, get_integer, get_integer_in_range};
use crate::question_generator::math_question_generator;

fn feed_routine(game_state: &mut LiveGameState, random_gen: &mut ThreadRng) {
    let pet = game_state.borrow_pet();

    loop {
        println!("{}", pet.format_stats());

        let (question, answer) = math_question_generator::generate(random_gen);
        match get_input(format!("?: {question} = ")).parse::<i32>() {
            Ok(user_guess) => {
                if user_guess == answer {
                    println!("Correct!");
                    pet.feed(game_state.food_per_correct);
                } else {
                    println!("Incorrect! Correct answer: {answer}");
                    pet.hurt(game_state.damage_per_wrong);
                }
            },
            Err(_) => {
                break;
            }
        }
    }

    println!("{}", pet.format_stats());
}

fn menu_dispatch(game_state: &mut LiveGameState, random_gen: &mut ThreadRng) -> bool {
    println!("
        1. Feed
        2. Stats
        3. Exit
    ");

    return match get_integer_in_range(1, 3, "Enter a menu option") {
        1 => {
            feed_routine(game_state, random_gen);
            true
        },
        2 => {
            true
        },
        3 => {
            false
        },
        _ => true,
    };
}

pub fn main_loop(game_state: &mut LiveGameState, random_gen: &mut ThreadRng) {
    loop {
        let keep_playing = menu_dispatch(game_state, random_gen);
        if !keep_playing {
            break;
        }
    }
}

