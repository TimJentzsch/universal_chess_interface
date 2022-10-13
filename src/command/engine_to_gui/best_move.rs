use std::fmt::Display;

use crate::chess::Move;

/// The engine has stopped searching and found the given move best in this position.
#[derive(Debug, PartialEq, Eq)]
pub struct BestMoveCommand {
    /// The best move in this position, according to the engine.
    pub best_move: Move,

    /// If given, the engine would like to ponder on this move.
    ///
    /// The engine must not start pondering automatically.
    pub ponder: Option<Move>,
}

impl BestMoveCommand {
    /// Create a new best move command with the given best move.
    pub fn new(best_move: Move) -> Self {
        Self {
            best_move,
            ponder: None,
        }
    }

    /// Create a new best move command with a suggestion to ponder on the given move.
    pub fn new_with_ponder(best_move: Move, ponder: Move) -> Self {
        Self {
            best_move,
            ponder: Some(ponder),
        }
    }
}

impl Display for BestMoveCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("bestmove {}", self.best_move);

        if let Some(ponder) = &self.ponder {
            result += &format!(" ponder {}", ponder);
        }

        write!(f, "{result}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{File, Move, Rank, Square};
    use rstest::rstest;

    #[rstest]
    #[case(
        BestMoveCommand::new(Move::new(
            Square::new(File::G, Rank::One),
            Square::new(File::F, Rank::Three)
        )),
        "bestmove g1f3"
    )]
    #[case(
        BestMoveCommand::new_with_ponder(
            Move::new(Square::new(File::G, Rank::One), Square::new(File::F, Rank::Three)),
            Move::new(Square::new(File::D, Rank::Eight), Square::new(File::F, Rank::Six))
        ),
        "bestmove g1f3 ponder d8f6"
    )]
    fn format_best_move_cmd(#[case] input: BestMoveCommand, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
