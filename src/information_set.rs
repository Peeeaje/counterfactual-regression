use crate::N_ACTIONS;

/// Information Set.
#[derive(Clone)]
pub struct InformationSet {
    /// The key of the information set, composed of card and history.
    pub key: String,
    /// The sum of regrets for each action.
    pub regret_sum: Vec<f32>,
    /// The sum of strategies for each action.
    pub strategy_sum: Vec<f32>,
    /// The probability of each action.
    pub strategy: Vec<f32>,
    /// The probability of reaching this information set.
    pub reach_pr: f32,
    /// The sum of reach probabilities over iterations.
    pub reach_pr_sum: f32,
}

impl InformationSet {
    /// Update strategy sum of information set.
    ///
    /// Update Formula:
    /// strategy_sum = strategy_sum + reach_pr * strategy
    pub fn update_strategy_sum(&mut self) {
        self.strategy_sum = self
            .strategy_sum
            .iter()
            .zip(self.strategy.iter())
            .map(|(&s_sum, &s)| s_sum + self.reach_pr * s)
            .collect::<Vec<f32>>()
    }

    /// Update reach probability sum of information set.
    ///
    /// Update Formula:
    /// reach_pr_sum = reach_pr_sum + reach_pr
    pub fn update_reach_pr_sum(&mut self) {
        self.reach_pr_sum += self.reach_pr;
    }

    /// Update strategy of information set.
    ///
    /// Update Formula:
    /// strategy = make_positive(regret_sum)
    /// strategy = normalize_strategy(strategy)
    pub fn update_strategy(&mut self) {
        let strategy = self.make_positive(&self.regret_sum);
        self.strategy = self.normalize_vector(&strategy);
    }

    /// Get average strategy of information set.
    ///
    /// Formula:
    /// total_strategy = strategy_sum / reach_pr_sum
    /// average_strategy = total_strategy / sum(total_strategy)
    pub fn get_average_strategy(&self) -> Vec<f32> {
        self.normalize_vector(&self.strategy_sum)
    }

    /// Make regret sum positive.
    /// If regret sum is negative, set it to 0.0.
    /// e.g. [-1.0, 2.0] -> [0.0, 2.0]
    pub fn make_positive(&self, regret_sum: &Vec<f32>) -> Vec<f32> {
        regret_sum
            .iter()
            .map(|&r| if r > 0.0 { r } else { 0.0 })
            .collect::<Vec<f32>>()
    }

    /// Normalize strategy.
    /// If sum of strategy is 0, set it to 1 / N_ACTIONS.
    fn normalize_vector(&self, vector: &Vec<f32>) -> Vec<f32> {
        let total: f32 = vector.iter().sum();
        if total > 0.0 {
            vector.iter().map(|&v| v / total).collect::<Vec<f32>>()
        } else {
            vec![1.0 / N_ACTIONS as f32; N_ACTIONS as usize]
        }
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

impl Default for InformationSet {
    fn default() -> Self {
        InformationSet {
            key: "".to_string(),
            regret_sum: vec![0.0; N_ACTIONS as usize],
            strategy_sum: vec![0.0; N_ACTIONS as usize],
            strategy: vec![1.0 / N_ACTIONS as f32; N_ACTIONS as usize],
            reach_pr: 0.0,
            reach_pr_sum: 0.0,
        }
    }
}
