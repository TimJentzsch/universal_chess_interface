use std::str::FromStr;

use super::ParseError;

/// A file of the board, denoted from a to h.
#[derive(Debug, PartialEq, Eq)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl FromStr for File {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(ParseError);
        }

        let file = match s {
            "a" => File::A,
            "b" => File::B,
            "c" => File::C,
            "d" => File::D,
            "e" => File::E,
            "f" => File::F,
            "g" => File::G,
            "h" => File::H,
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
    #[case("a", File::A)]
    #[case("b", File::B)]
    #[case("c", File::C)]
    #[case("d", File::D)]
    #[case("e", File::E)]
    #[case("f", File::F)]
    #[case("g", File::G)]
    #[case("h", File::H)]
    fn parse_file_ok(#[case] input: &str, #[case] expected: File) {
        let actual = input.parse();
        assert_eq!(actual, Ok(expected));
    }

    #[rstest]
    #[case("A")]
    #[case("B")]
    #[case("C")]
    #[case("D")]
    #[case("E")]
    #[case("F")]
    #[case("G")]
    #[case("H")]
    #[case("")]
    #[case("aa")]
    fn parse_file_err(#[case] input: &str) {
        let actual = input.parse::<File>();
        assert!(matches!(actual, Err(_)));
    }
}
