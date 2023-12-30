use std::collections::HashMap;
use rand::prelude::ThreadRng;
use rand::Rng;

// type BinaryOp = fn(x: i32, y: i32) -> i32;
// type BinaryOpMap = HashMap<String, (i32, i32, i32, i32, BinaryOp)>;
// //
//
//
// struct MathQuestionGenerator {
//     binary_ops: BinaryOpMap,
//     random_gen: ThreadRng,
// }
//
// impl MathQuestionGenerator {
//     fn new_with_ops(binary_ops: BinaryOpMap, random_gen: ThreadRng) -> Self {
//         return Self {
//             binary_ops: binary_ops,
//             random_gen: random_gen,
//         };
//     }
// }

const MIN_N: i32 = 1;
const MAX_N: i32 = 9;

fn generate_operands(random_gen: &mut ThreadRng, min_n: i32, max_n: i32) -> (i32, i32) {
    return (
      random_gen.gen_range(min_n..=max_n),
      random_gen.gen_range(min_n..=max_n),
    );
}

pub fn generate(random_gen: &mut ThreadRng) -> (String, i32) {
    let x = random_gen.gen_range(MIN_N..=MAX_N);
    let y = random_gen.gen_range(MIN_N..=MAX_N);

    return match random_gen.gen_range(0..3) {
        0 => (format!("{x} + {y}"), x + y),
        1 => (format!("{x} - {y}"), x - y),
        2 => (format!("{x} * {y}"), x * y),
        _ => ("INVALID".to_string(), 0),
    }
}