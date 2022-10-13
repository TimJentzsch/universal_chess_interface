use std::{
    fmt::Display,
    str::{FromStr, SplitAsciiWhitespace},
};

pub mod engine_to_gui;
pub mod gui_to_engine;

/// A raw UCI command.
pub struct RawCommand {
    /// The full content of the command.
    pub content: String,
}

impl RawCommand {
    /// Create a new command with the given content.
    pub fn new<S>(content: S) -> Self
    where
        S: Into<String>,
    {
        RawCommand {
            content: content.into(),
        }
    }

    /// Get the whitespace-separated tokens of the command.
    pub fn tokens(&self) -> SplitAsciiWhitespace<'_> {
        self.content.split_ascii_whitespace()
    }

    /// Get the name of the command.
    ///
    /// The name is the first token.
    pub fn name(&self) -> String {
        self.tokens()
            .next()
            .expect("A command must have at least one token")
            .to_string()
    }

    /// Get the arguments of the command.
    ///
    /// The arguments are all tokens but the first.
    pub fn arguments(&self) -> SplitAsciiWhitespace<'_> {
        let mut tokens = self.tokens();
        // Discard the name
        tokens.next();
        tokens
    }
}

impl Display for RawCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

/// An error has occured while parsing a UCI command.
#[derive(Debug, PartialEq, Eq)]
pub struct CommandParseError;

impl FromStr for RawCommand {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RawCommand::new(s))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("uci", "uci")]
    #[case("stop", "stop")]
    #[case("info depth 12 nodes 123456 nps 100000", "info")]
    fn get_name(#[case] input: String, #[case] expected: String) {
        let command = input
            .parse::<RawCommand>()
            .expect("This is a valid command");

        assert_eq!(command.name(), expected);
    }
}
