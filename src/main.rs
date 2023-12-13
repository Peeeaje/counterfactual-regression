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

/// counterfactual regret minimization algorithm
fn cfr(
    i_map: HashMap<String, InformationSet>,
    history: String,
    card_1: i32,
    card_2: i32,
    pr_1: f32,
    pr_2: f32,
    pr_c: f32,
) -> i32 {
    if is_chance_node(history) {
        return chance_util(i_map);
    }

    if is_terminal(history) {
        return terminal_util(history, card_1, card_2);
    }

    let n: usize = history.len();
    let is_player_1: bool = n % 2 == 0;

    if is_player_1 {
        let info_set: InformationSet = get_info_set(i_map, card_1, history);
    } else {
        let info_set: InformationSet = get_info_set(i_map, card_2, history);
    }

    strategy = info_set.strategy;

    if is_player_1 {
        info_set.reach_pr += pr_1;
    } else {
        info_set.reach_pr += pr_2;
    }

    // counterfactual utility per action
    action_utils =

    return 1;
}
