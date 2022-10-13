use std::{fmt::Display, str::FromStr};

use super::{ParseError, PromotionPiece, Square};

/// A move on the board, in long algebraic notation.
#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    /// The source square where the moving piece started.
    source: Square,

    /// The target square where the moving piece is going to.
    target: Square,

    /// The piece that the pawn promoted to, if the move is a promotion.
    promotion: Option<PromotionPiece>,
}

impl Move {
    /// Create a new move without a promotion.
    pub fn new(source: Square, target: Square) -> Self {
        Move {
            source,
            target,
            promotion: None,
        }
    }

    /// Create a new move with a promoting pawn.
    pub fn new_with_promotion(source: Square, target: Square, promotion: PromotionPiece) -> Self {
        Move {
            source,
            target,
            promotion: Some(promotion),
        }
    }
}

impl FromStr for Move {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 && s.len() != 5 {
            return Err(ParseError);
        }

        let source = s[(0..2)].parse::<Square>()?;
        let target = s[(2..4)].parse::<Square>()?;

        let promotion = if s.len() == 5 {
            Some(s[4..5].parse::<PromotionPiece>()?)
        } else {
            None
        };

        Ok(Move {
            source,
            target,
            promotion,
        })
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(promotion) = &self.promotion {
            write!(f, "{}{}{promotion}", self.source, self.target)
        } else {
            write!(f, "{}{}", self.source, self.target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{File, Rank};
    use rstest::rstest;

    #[rstest]
    #[case(
        "e2e4",
        Move::new(Square::new(File::E, Rank::Two), Square::new(File::E, Rank::Four))
    )]
    #[case(
        "e7e2",
        Move::new(Square::new(File::E, Rank::Seven), Square::new(File::E, Rank::Two))
    )]
    #[case(
        "e1g1",
        Move::new(Square::new(File::E, Rank::One), Square::new(File::G, Rank::One))
    )]
    #[case(
        "e7e8q",
        Move::new_with_promotion(
            Square::new(File::E, Rank::Seven),
            Square::new(File::E, Rank::Eight),
            PromotionPiece::Queen
        )
    )]
    fn parse_move_ok(#[case] input: &str, #[case] expected: Move) {
        let actual = input.parse();
        assert_eq!(actual, Ok(expected));
    }

    #[rstest]
    #[case("")]
    #[case("a")]
    #[case("1")]
    #[case("a1")]
    #[case("E2e4")]
    #[case("e2E4")]
    #[case("a0a1")]
    #[case("e7e8p")]
    #[case("e7e8k")]
    #[case("e7e8qn")]
    fn parse_move_err(#[case] input: &str) {
        let actual = input.parse::<Move>();
        assert!(matches!(actual, Err(_)));
    }

    #[rstest]
    #[case(
        Move::new(Square::new(File::E, Rank::Two), Square::new(File::E, Rank::Four)),
        "e2e4"
    )]
    #[case(
        Move::new(Square::new(File::E, Rank::Seven), Square::new(File::E, Rank::Two)),
        "e7e2"
    )]
    #[case(
        Move::new(Square::new(File::E, Rank::One), Square::new(File::G, Rank::One)),
        "e1g1"
    )]
    #[case(
        Move::new_with_promotion(
            Square::new(File::E, Rank::Seven),
            Square::new(File::E, Rank::Eight),
            PromotionPiece::Queen
        ),
        "e7e8q"
    )]
    fn format_move(#[case] input: Move, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
