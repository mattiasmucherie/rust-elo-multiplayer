pub struct EloRank {
    pub k: i32,
    pub players: Vec<f64>,
}
impl Default for EloRank {
    fn default() -> EloRank {
        EloRank {
            k: 32,
            players: vec![1000f64, 1000f64],
        }
    }
}

impl EloRank {
    fn calculate_expected(&self) -> Vec<f64> {
        let amount_of_players = self.players.len() as f64;
        let mut expected_array: Vec<f64> = vec![];
        for (i, _) in self.players.iter().enumerate() {
            let sum = self.players.iter().enumerate().fold(0f64, |s, (j, p)| {
                if i == j {
                    s
                } else {
                    s + 1.0 / (1.0 + 10f64.powf((p - self.players[i]) / 400f64))
                }
            });
            let expected = sum / ((amount_of_players * (amount_of_players - 1f64)) / 2f64);
            expected_array.push(expected)
        }
        return expected_array;
    }

    fn calculate_scores(&self) -> Vec<f64> {
        let amount_of_players = self.players.len() as f64;
        let mut scores: Vec<f64> = vec![];
        for (i, _) in self.players.iter().enumerate() {
            let score = (amount_of_players - (i as f64 + 1f64))
                / (amount_of_players * (amount_of_players - 1f64) / 2f64);
            scores.push(score);
        }
        scores
    }

    pub fn calculate(&self) -> Vec<f64> {
        let amount_of_players = self.players.len() as f64;
        let expected = self.calculate_expected();
        let scores = self.calculate_scores();
        let mut new_elo: Vec<f64> = vec![];
        for (i, p) in self.players.iter().enumerate() {
            new_elo
                .push(p + (self.k as f64) * (amount_of_players - 1f64) * (scores[i] - expected[i]));
        }
        new_elo
    }
}

#[cfg(test)]
mod tests {
    use crate::EloRank;
    #[test]
    fn calculate_ratings() {
        let players: Vec<f64> = vec![1000.0, 1000.0, 1000.0, 1000.0];
        let elo = EloRank {
            players,
            ..Default::default()
        };
        let new_elo = elo.calculate();
        assert_eq!(new_elo, vec![1024.0, 1008.0, 992.0, 976.0]);
    }
}
