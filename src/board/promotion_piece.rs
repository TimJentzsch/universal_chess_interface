use std::str::FromStr;

use super::ParseError;

/// A piece that a pawn can promote to.
#[derive(Debug, PartialEq, Eq)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
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
}
