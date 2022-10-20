pub struct EloRank {
    pub k: i32,
    pub players: Vec<f64>,
    pub score_base: f64,
}
impl Default for EloRank {
    fn default() -> EloRank {
        EloRank {
            k: 32,
            players: vec![1000f64, 1000f64],
            score_base: 1.0,
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
        expected_array
    }

    fn calculate_scores(&self) -> Vec<f64> {
        let amount_of_players = self.players.len() as f64;
        let mut scores: Vec<f64> = vec![];
        for (i, _) in self.players.iter().enumerate() {
            let score: f64;
            if self.score_base == 1.0 {
                score = (amount_of_players - (i as f64 + 1f64))
                    / (amount_of_players * (amount_of_players - 1f64) / 2f64);
            } else {
                score = (self.score_base.powf(amount_of_players - (i + 1) as f64) - 1f64)
                    / (self.players.iter().enumerate().fold(0.0, |s, (j, _)| {
                        s + (self.score_base.powf(amount_of_players - (j + 1) as f64) - 1f64)
                    }));
            }
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
    fn test_elo() {
        let elo = EloRank {
            players: vec![897.0, 978.0],
            ..Default::default()
        };
        assert_eq!(elo.calculate(), vec![916.6640435522738, 958.3359564477262]);

        let elo = EloRank {
            players: vec![978.0, 897.0],
            ..Default::default()
        };
        assert_eq!(elo.calculate(), vec![990.3359564477262, 884.6640435522738]);

        let elo = EloRank {
            players: vec![1000.0, 1000.0, 1000.0],
            ..Default::default()
        };
        assert_eq!(
            elo.calculate(),
            vec![1021.3333333333334, 1000.0, 978.6666666666666]
        );

        let elo = EloRank {
            players: vec![1000.0, 1000.0, 1000.0],
            score_base: 1.5,
            ..Default::default()
        };
        assert_eq!(
            elo.calculate(),
            [1024.3809523809523, 996.952380952381, 978.6666666666666]
        );

        let elo = EloRank {
            players: vec![897.0, 978.0, 982.0, 995.0, 1017.0, 1034.0, 1096.0],
            ..Default::default()
        };
        assert_eq!(
            elo.calculate(),
            vec![
                933.4271852776218,
                998.2520240277877,
                992.7510517787325,
                995.4420183323965,
                1006.3245067439552,
                1012.6627230848082,
                1060.140490754698
            ]
        );
    }
}
