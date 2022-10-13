use std::{fmt::Display, str::FromStr};

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

        let file = s[(0..1)].parse::<File>()?;
        let rank = s[(1..2)].parse::<Rank>()?;

        Ok(Square::new(file, rank))
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file, self.rank)
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

    #[rstest]
    #[case(Square::new(File::A, Rank::One), "a1")]
    #[case(Square::new(File::C, Rank::Two), "c2")]
    #[case(Square::new(File::D, Rank::Six), "d6")]
    #[case(Square::new(File::H, Rank::Eight), "h8")]
    fn format_square(#[case] input: Square, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
