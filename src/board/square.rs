use std::str::FromStr;

use super::{File, ParseError, Rank};

/// A square on the chess board.
#[derive(Debug, PartialEq, Eq)]
pub struct Square {
    /// The file that the square is on.
    pub file: File,

    /// The rank that the square is on.
    pub rank: Rank,
}

impl Square {
    /// Create a new square with the given file and rank.
    pub const fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }
}

impl FromStr for Square {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseError);
        }

        let file = s[(0..1)].parse::<File>();
        let rank = s[(1..2)].parse::<Rank>();

        match (file, rank) {
            (Ok(file), Ok(rank)) => Ok(Square { file, rank }),
            _ => Err(ParseError),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("a1", Square::new(File::A, Rank::One))]
    #[case("c2", Square::new(File::C, Rank::Two))]
    #[case("d6", Square::new(File::D, Rank::Six))]
    #[case("h8", Square::new(File::H, Rank::Eight))]
    fn parse_square_ok(#[case] input: &str, #[case] expected: Square) {
        let actual = input.parse();
        assert_eq!(actual, Ok(expected));
    }

    #[rstest]
    #[case("a0")]
    #[case("A1")]
    #[case("i2")]
    #[case("c33")]
    #[case("a3b5")]
    fn parse_square_err(#[case] input: &str) {
        let actual = input.parse::<Square>();
        assert!(matches!(actual, Err(_)));
    }
}
