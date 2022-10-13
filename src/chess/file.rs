use std::{fmt::Display, str::FromStr};

use super::ParseError;

/// A file of the board, denoted from a to h.
#[derive(Debug, PartialEq, Eq)]
pub enum File {
    /// The first file of the board, denoted `a`.
    A,

    /// The second file of the board, denoted `b`.
    B,

    /// The third file of the board, denoted `c`.
    C,

    /// The forth file of the board, denoted `d`.
    D,

    /// The fifth file of the board, denoted `e`.
    E,

    /// The sixth file of the board, denoted `f`.
    F,

    /// The seventh file of the board, denoted `g`.
    G,

    /// The eigth file of the board, denoted `h`.
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

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = match self {
            File::A => "a",
            File::B => "b",
            File::C => "c",
            File::D => "d",
            File::E => "e",
            File::F => "f",
            File::G => "g",
            File::H => "h",
        };

        write!(f, "{file}")
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
    #[case("1")]
    #[case("c5")]
    fn parse_file_err(#[case] input: &str) {
        let actual = input.parse::<File>();
        assert!(matches!(actual, Err(_)));
    }

    #[rstest]
    #[case(File::A, "a")]
    #[case(File::B, "b")]
    #[case(File::C, "c")]
    #[case(File::D, "d")]
    #[case(File::E, "e")]
    #[case(File::F, "f")]
    #[case(File::G, "g")]
    #[case(File::H, "h")]
    fn format_file(#[case] input: File, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
