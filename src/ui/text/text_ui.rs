use rand::rngs::ThreadRng;
use crate::game_state::{GameTweaks, LiveGameState};
use crate::pet::Pet;
use crate::text_util::{get_input, get_integer, get_integer_in_range};
use crate::question_generator::math_question_generator;
use crate::shop;

fn feed_routine(pet: &mut Pet, food_per_correct: f64, damage_per_wrong: f64, random_gen: &mut ThreadRng) -> (u32, u32) {
    let mut n_correct = 0;
    let mut n_wrong = 0;
    loop {
        println!("{}\n", pet.format_stats());

        let (question, answer) = math_question_generator::generate(random_gen);
        match get_input(format!("?: {question} = ")).parse::<i32>() {
            Ok(user_guess) => {
                if user_guess == answer {
                    println!("Correct!");
                    pet.feed(food_per_correct);
                    n_correct += 1;
                } else {
                    println!("Incorrect! Correct answer: {answer}");
                    pet.hurt(damage_per_wrong);
                    n_wrong += 1;
                }
            },
            Err(_) => {
                break;
            }
        }

        if pet.is_dead() {
            break;
        }
    }

    println!("{}", pet.format_stats());
    return (n_correct, n_wrong);
}

fn shop_routine(game_state: &mut LiveGameState) -> () {
    let inventory = shop::get_shop_inventory();

    loop {
        println!("{}", game_state.format_stats());
        println!("{}", shop::format_inventory(&inventory));

        match get_input("# to buy: ").parse::<usize>() {
            Ok(i) => {
                match inventory.get(i - 1) {
                    Some(item) => {
                        let bought = item.buy_and_apply(game_state);
                        if bought == false {
                            println!("Not enough money to buy!");
                        }
                    },
                    None => {
                        return;
                    }
                }
            },
            Err(_) => {
                return;
            }
        }
    }
}

fn menu_dispatch(game_state: &mut LiveGameState, random_gen: &mut ThreadRng) -> bool {
    println!("{}", game_state.format_stats());

    println!("
    1. Feed
    2. Shop
    3. Exit
    ");

    return match get_integer_in_range(1, 3, "Enter a menu option: ") {
        1 => {
            let LiveGameState { pet, tweaks: GameTweaks { food_per_correct, damage_per_wrong, ..}, ..} = game_state;
            feed_routine(pet, *food_per_correct, *damage_per_wrong, random_gen);
            true
        },
        2 => {
            shop_routine(game_state);
            true
        },
        3 => {
            false
        },
        _ => true,
    };
}

pub fn main_loop(game_state: &mut LiveGameState, random_gen: &mut ThreadRng) {
    game_state.account_for_elapsed_time();

    loop {
        if game_state.is_game_over() {
            break;
        }

        let keep_playing = menu_dispatch(game_state, random_gen);
        game_state.account_for_elapsed_time();

        if keep_playing == false {
            game_state.save_state();
            break;
        }
    }
}

