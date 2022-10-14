use std::fmt::Display;

use super::Move;

/// A line of moves.
#[derive(Debug, PartialEq, Eq)]
pub struct MoveLine(pub Vec<Move>);

impl MoveLine {
    /// Determines if there are any moves in this line.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The number of moves in this line.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Add a move to the end of the line.
    pub fn push(&mut self, mv: Move) {
        self.0.push(mv)
    }
}

impl From<Vec<Move>> for MoveLine {
    fn from(moves: Vec<Move>) -> Self {
        MoveLine(moves)
    }
}

impl From<MoveLine> for Vec<Move> {
    fn from(moves: MoveLine) -> Self {
        moves.0
    }
}

impl Display for MoveLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let move_strings: Vec<_> = self.0.iter().map(|mv| mv.to_string()).collect();
        let output = move_strings.join(" ");

        write!(f, "{output}")
    }
}

#[cfg(test)]
mod tests {
    use super::super::Move;
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![
        Move::new(
            Square::new(File::E, Rank::Two),
            Square::new(File::E, Rank::Four),
        ),
        Move::new(
            Square::new(File::E, Rank::One),
            Square::new(File::G, Rank::One),
        ),
    ], "e2e4 e1g1")]
    fn format_line(#[case] input: MoveLine, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
