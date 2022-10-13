use std::str::FromStr;

use super::ParseError;

/// A rank of the board, denoted from 1 to 8.
#[derive(Debug, PartialEq, Eq)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl FromStr for Rank {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseError);
        }

        let rank = match s {
            "1" => Rank::One,
            "2" => Rank::Two,
            "3" => Rank::Three,
            "4" => Rank::Four,
            "5" => Rank::Five,
            "6" => Rank::Six,
            "7" => Rank::Seven,
            "8" => Rank::Eight,
            _ => return Err(ParseError),
        };

        Ok(rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", Rank::One)]
    #[case("2", Rank::Two)]
    #[case("3", Rank::Three)]
    #[case("4", Rank::Four)]
    #[case("5", Rank::Five)]
    #[case("6", Rank::Six)]
    #[case("7", Rank::Seven)]
    #[case("8", Rank::Eight)]
    fn parse_rank_ok(#[case] input: &str, #[case] expected: Rank) {
        let actual = input.parse();
        assert_eq!(actual, Ok(expected));
    }

    #[rstest]
    #[case("a")]
    #[case("0")]
    #[case("")]
    #[case("11")]
    #[case("b3")]
    fn parse_file_err(#[case] input: &str) {
        let actual = input.parse::<Rank>();
        assert!(matches!(actual, Err(_)));
    }
}
