use std::{fmt::Display, str::FromStr};

use super::ParseError;

/// A piece that a pawn can promote to.
#[derive(Debug, PartialEq, Eq)]
pub enum PromotionPiece {
    /// Promoting to a knight, denoted 'n'.
    Knight,

    /// Promoting to a bishop, denoted 'b'.
    Bishop,

    /// Promoting to a rook, denoted 'r'.
    Rook,

    /// Promoting to a queen, denoted 'q'.
    Queen,
}

impl FromStr for PromotionPiece {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseError);
        }

        let file = match s {
            "n" => PromotionPiece::Knight,
            "b" => PromotionPiece::Bishop,
            "r" => PromotionPiece::Rook,
            "q" => PromotionPiece::Queen,
            _ => return Err(ParseError),
        };

        Ok(file)
    }
}

impl Display for PromotionPiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece = match self {
            PromotionPiece::Knight => "n",
            PromotionPiece::Bishop => "b",
            PromotionPiece::Rook => "r",
            PromotionPiece::Queen => "q",
        };

        write!(f, "{piece}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("n", PromotionPiece::Knight)]
    #[case("b", PromotionPiece::Bishop)]
    #[case("r", PromotionPiece::Rook)]
    #[case("q", PromotionPiece::Queen)]
    fn parse_promotion_piece_ok(#[case] input: &str, #[case] expected: PromotionPiece) {
        let actual = input.parse();
        assert_eq!(actual, Ok(expected));
    }

    #[rstest]
    #[case("N")]
    #[case("B")]
    #[case("R")]
    #[case("Q")]
    #[case("p")]
    #[case("k")]
    #[case("")]
    #[case("nn")]
    #[case("br")]
    fn parse_promotion_piece_err(#[case] input: &str) {
        let actual = input.parse::<PromotionPiece>();
        assert!(matches!(actual, Err(_)));
    }

    #[rstest]
    #[case(PromotionPiece::Knight, "n")]
    #[case(PromotionPiece::Bishop, "b")]
    #[case(PromotionPiece::Rook, "r")]
    #[case(PromotionPiece::Queen, "q")]
    fn format_promotion_piece(#[case] input: PromotionPiece, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
