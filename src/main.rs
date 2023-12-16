use std::collections::HashMap;

const N_ACTIONS: i32 = 2;
const N_ITERATIONS: i32 = 100;
const N_CARDS: i32 = 3;

#[derive(Clone)]
struct InformationSet {
    key: String,
    regret_sum: Vec<f32>,
    strategy_sum: Vec<f32>,
    strategy: Vec<f32>,
    reach_pr: f32,
    reach_pr_sum: f32,
}

impl InformationSet {
    fn next_strategy(&mut self) {
        self.strategy_sum = self
            .strategy_sum
            .iter()
            .zip(self.strategy.iter())
            .map(|(&s_sum, &s)| s_sum + self.reach_pr * s)
            .collect::<Vec<f32>>();
        self.strategy = self.calc_strategy();
        self.reach_pr_sum += self.reach_pr;
        self.reach_pr = 0.0;
    }
    fn calc_strategy(&self) -> Vec<f32> {
        let mut strategy = self.make_positive(&self.regret_sum);
        let total: f32 = strategy.iter().sum();
        if total > 0.0 {
            for s in strategy.iter_mut() {
                *s /= total;
            }
        } else {
            for s in strategy.iter_mut() {
                *s = 1.0 / N_ACTIONS as f32;
            }
        }
        strategy
    }
    fn get_average_strategy(&self) -> Vec<f32> {
        let total_strategy = self
            .strategy_sum
            .iter()
            .map(|s| s / self.reach_pr_sum)
            .map(|s| if s > 0.001 { s } else { 0.0 })
            .collect::<Vec<f32>>();

        let total: f32 = total_strategy.iter().sum();
        let mut average_strategy = total_strategy;
        average_strategy.iter_mut().for_each(|s| *s /= total as f32);
        average_strategy
    }
    fn make_positive(&self, regret_sum: &Vec<f32>) -> Vec<f32> {
        let mut positive_regret_sum = regret_sum.clone();
        positive_regret_sum
            .iter_mut()
            .for_each(|r| *r = if *r > 0.0 { *r } else { 0.0 });
        positive_regret_sum
    }
}

impl std::fmt::Display for InformationSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let strategies = self.get_average_strategy();
        write!(
            f,
            "{} {} {}",
            self.key,
            strategies[0].to_string(),
            strategies[1].to_string(),
        )
    }
}

/// counterfactual regret minimizationのiterationを行う
fn main() {
    let mut i_map: HashMap<String, InformationSet> = HashMap::new();
    let mut expected_game_value = 0.0;

    for _i in 1..N_ITERATIONS {
        expected_game_value += cfr(&mut i_map, String::from(""), -1, -1, 1.0, 1.0, 1.0);
        for (_key, value) in i_map.iter_mut() {
            value.next_strategy();
        }
    }
    expected_game_value /= N_ITERATIONS as f32;
    display_result(expected_game_value, &i_map);
}

/// counterfactual regret minimization algorithm
fn cfr(
    mut i_map: &mut HashMap<String, InformationSet>,
    history: String,
    card_1: i32,
    card_2: i32,
    pr_1: f32,
    pr_2: f32,
    pr_c: f32,
) -> f32 {
    if is_chance_node(&history) {
        return chance_util(i_map);
    }

    if is_terminal(&history) {
        return terminal_util(&history, card_1, card_2);
    }

    let n: usize = history.len();
    let is_player_1: bool = n % 2 == 0;

    let mut info_set: InformationSet;
    if is_player_1 {
        info_set = get_info_set(&mut i_map, card_1, &history);
    } else {
        info_set = get_info_set(&mut i_map, card_2, &history);
    }

    let strategy = info_set.strategy;

    if is_player_1 {
        info_set.reach_pr += pr_1;
    } else {
        info_set.reach_pr += pr_2;
    }

    // counterfactual utility per action
    let mut action_utils = vec![0.0; N_ACTIONS as usize];

    for (i, action) in vec!["c", "b"].iter().enumerate() {
        let next_history = format!("{}{}", history, action);

        if is_player_1 {
            action_utils[i] = -cfr(
                &mut i_map,
                next_history,
                card_1,
                card_2,
                pr_1 * strategy[i],
                pr_2,
                pr_c,
            );
        } else {
            action_utils[i] = -cfr(
                &mut i_map,
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

fn is_chance_node(history: &str) -> bool {
    return history == "";
}

fn chance_util(i_map: &mut HashMap<String, InformationSet>) -> f32 {
    let mut expected_value = 0.0;
    let n_possibilities = 6;
    for _i in 0..N_CARDS {
        for _j in 0..N_CARDS {
            if _i == _j {
                continue;
            }
            expected_value += cfr(
                i_map,
                "rr".to_string(),
                _i,
                _j,
                1.0,
                1.0,
                1.0 / n_possibilities as f32,
            );
        }
    }
    expected_value / n_possibilities as f32
}

fn is_terminal(history: &str) -> bool {
    let possible_terminal = vec!["rrcc", "rrcbc", "rrcbb", "rrbc", "rrbb"];
    possible_terminal.contains(&history)
}

fn terminal_util(history: &str, card_1: i32, card_2: i32) -> f32 {
    let player_card = if history.len() / 2 == 0 {
        card_1
    } else {
        card_2
    };
    let opponent_card = if history.len() / 2 == 0 {
        card_2
    } else {
        card_1
    };

    if (history == "rrcbc") | (history == "rrbc") {
        return 1.0;
    } else if history == "rrcc" {
        return if player_card > opponent_card {
            1.0
        } else {
            -1.0
        };
    } else {
        return if player_card > opponent_card {
            2.0
        } else {
            -2.0
        };
    }
}

fn card_str(card: i32) -> String {
    match card {
        0 => return "J".to_string(),
        1 => return "Q".to_string(),
        2 => return "K".to_string(),
        _ => return "".to_string(),
    }
}

fn get_info_set(
    i_map: &mut HashMap<String, InformationSet>,
    card: i32,
    history: &str,
) -> InformationSet {
    let key = format!("{} {}", card_str(card), history);
    let info_set = i_map.get(&key);

    if info_set.is_none() {
        let new_info_set = InformationSet {
            key: key.clone(),
            regret_sum: vec![0.0; N_ACTIONS as usize],
            strategy_sum: vec![0.0; N_ACTIONS as usize],
            strategy: vec![0.0; N_ACTIONS as usize],
            reach_pr: 0.0,
            reach_pr_sum: 0.0,
        };
        i_map.insert(key.clone(), new_info_set);
    }
    return i_map.get(&key.clone()).unwrap().clone();
}

fn display_result(expected_game_value: f32, i_map: &HashMap<String, InformationSet>) {
    println!("player 1 expected game value: {}", expected_game_value);
    println!("player 2 expected game value: {}", -expected_game_value);

    println!("player 1 strategy:");
    let mut items: Vec<_> = i_map.iter().collect();
    items.sort_by(|a, b| a.0.cmp(&b.0));
    for (key, value) in items.iter() {
        if key.len() % 2 == 0 {
            println!("{}", value);
        }
    }

    println!("player 2 strategy:");
    for (key, value) in items.iter() {
        if key.len() % 2 == 1 {
            println!("{}", value);
        }
    }
}
