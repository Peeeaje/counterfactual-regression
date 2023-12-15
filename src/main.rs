use std::collections::HashMap;

const N_ACTIONS: i32 = 2;
const N_CARDS: i32 = 3;

struct InformationSet {
    strategy: Vec<f32>,
    regret_sum: Vec<f32>,
    reach_pr: f32,
}

impl InformationSet {
    fn next_strategy(&self) -> Vec<f32> {
        return self.strategy;
    }
}

/// counterfactual regret minimizationのiterationを行う
fn main() {
    let mut i_map: HashMap<String, InformationSet> = HashMap::new();
    const N_ITERATIONS: i32 = 10000;
    let mut expected_game_value = 0.0;

    for i in 1..N_ITERATIONS {
        expected_game_value += cfr(i_map, String::from(""), -1, -1, 1.0, 1.0, 1.0);
        for (key, value) in i_map.iter_mut() {
            value.next_strategy();
        }
    }

    expected_game_value /= N_ITERATIONS as f32;

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
) -> f32 {
    if is_chance_node(history) {
        return chance_util(i_map);
    }

    if is_terminal(history) {
        return terminal_util(history, card_1, card_2);
    }

    let n: usize = history.len();
    let is_player_1: bool = n % 2 == 0;

    let mut info_set: InformationSet;
    if is_player_1 {
        let info_set = get_info_set(i_map, card_1, history);
    } else {
        let info_set = get_info_set(i_map, card_2, history);
    }

    let strategy = info_set.strategy;

    if is_player_1 {
        info_set.reach_pr += pr_1;
    } else {
        info_set.reach_pr += pr_2;
    }

    // counterfactual utility per action
    let action_utils = vec![0.0; N_ACTIONS as usize];

    for (i, action) in vec!["c", "b"].iter().enumerate() {
        let next_history = format!("{}{}", history, action);

        if is_player_1 {
            action_utils[i] = -cfr(
                i_map,
                next_history,
                card_1,
                card_2,
                pr_1 * strategy[i],
                pr_2,
                pr_c,
            );
        } else {
            action_utils[i] = -cfr(
                i_map,
                next_history,
                card_1,
                card_2,
                pr_1,
                pr_2 * strategy[i],
                pr_c,
            );
        }
    }

    // Utility of information set
    let util = action_utils
        .iter()
        .zip(strategy.iter())
        .map(|(&u, &s)| u * s)
        .sum();

    let regrets = action_utils.iter().map(|u| u - util);

    if is_player_1 {
        info_set
            .regret_sum
            .iter_mut()
            .zip(regrets)
            .for_each(|(r, u)| *r += pr_2 * pr_c * u);
    } else {
        info_set
            .regret_sum
            .iter_mut()
            .zip(regrets)
            .for_each(|(r, u)| *r += pr_1 * pr_c * u);
    }

    return util;
}

fn is_chance_node(history: String) -> bool {
    return history == "";
}

fn chance_util(i_map: HashMap<String, InformationSet>) -> f32 {
    let mut expected_value = 0.0;
    let n_possibilities = 6;
    for i in 1..N_CARDS {
        for j in 1..N_CARDS {
            expected_value += cfr(
                i_map,
                "rr".to_string(),
                i,
                j,
                1.0,
                1.0,
                1.0 / n_possibilities as f32,
            );
        }
    }
    expected_value / n_possibilities as f32
}

fn is_terminal(history: String) -> bool {
    let possible_terminal = vec!["rrcc", "rrcbc", "rrcbb", "rrbc", "rrbb"];
    possible_terminal.contains(&history.as_str())
}
