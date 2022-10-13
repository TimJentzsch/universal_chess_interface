use std::fmt::Display;

/// The current (estimated) evaluation of the game.
pub enum Evaluation {
    /// The current player can give checkmate in the given number of moves.
    ///
    /// Note that the number represents moves, not plies.
    PlayerCheckmate(u8),

    /// The opponent can give checkmate in the given number of moves.
    ///
    /// Note that the number represents moves, not plies.
    OpponentCheckmate(u8),

    /// The evaluation of the current position in centipawns.
    ///
    /// A positive value represents an advantage for the current player,
    /// a negative value represents an advantage for the opponent.
    Centipawns(i32),
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Evaluation::PlayerCheckmate(moves) => format!("mate {moves}"),
            Evaluation::OpponentCheckmate(moves) => format!("mate -{moves}"),
            Evaluation::Centipawns(cp) => format!("cp {cp}"),
        };

        write!(f, "{output}")
    }
}

/// The current game score.
pub struct Score {
    /// The evaluation of the position.
    eval: Evaluation,

    /// Indicates whether the score is just a lower bound.
    is_lower_bound: bool,

    /// Indicates whether the score is just an upper bound.
    is_upper_bound: bool,
}

impl Score {
    /// Create a new score that is not a lower nor upper bound.
    pub fn new(eval: Evaluation) -> Self {
        Self {
            eval,
            is_lower_bound: false,
            is_upper_bound: false,
        }
    }

    /// Create a new score that is just a lower bound.
    pub fn lower_bound(eval: Evaluation) -> Self {
        Self {
            eval,
            is_lower_bound: true,
            is_upper_bound: false,
        }
    }

    /// Create a new score that is just an upper bound.
    pub fn uppper_bound(eval: Evaluation) -> Self {
        Self {
            eval,
            is_lower_bound: false,
            is_upper_bound: true,
        }
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("score {}", self.eval);

        if self.is_lower_bound {
            output += " lowerbound";
        }

        if self.is_upper_bound {
            output += " upperbound"
        }

        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(Score::new(Evaluation::PlayerCheckmate(2)), "score mate 2")]
    #[case(Score::new(Evaluation::OpponentCheckmate(4)), "score mate -4")]
    #[case(Score::new(Evaluation::Centipawns(12)), "score cp 12")]
    #[case(Score::new(Evaluation::Centipawns(-60)), "score cp -60")]
    #[case(
        Score::lower_bound(Evaluation::Centipawns(35)),
        "score cp 35 lowerbound"
    )]
    #[case(
        Score::uppper_bound(Evaluation::Centipawns(87)),
        "score cp 87 upperbound"
    )]
    fn format_score(#[case] input: Score, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
