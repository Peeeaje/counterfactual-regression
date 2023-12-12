use std::collections::HashMap;


const N_ACTIONS: i32 = 2;
const N_CARDS: i32 = 3;

/// counterfactual regret minimizationのiterationを行う
fn main() {
    let mut i_map = HashMap::new();
    i_map.insert("aaa", 0);
    const N_ITERATIONS: i32 = 10000;
    let mut expected_game_value: i32 = 0;

    for i in 1..N_ITERATIONS {
        expected_game_value += cfr(i_map);
        for (key, value) in i_map.iter_mut() {
            value.next_strategy();
        }
    }

    expected_game_value /= N_ITERATIONS;

    println!("expected game value: {}", expected_game_value);
}
