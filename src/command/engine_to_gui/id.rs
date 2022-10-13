use std::fmt::Display;

/// Command to identify the engine to the GUI.
#[derive(Debug, PartialEq, Eq)]
pub enum IdCommand {
    /// The name of the engine.
    Name(String),

    /// The author of the engine.
    Author(String),
}

impl IdCommand {
    /// Construct a new ID command with the given name.
    pub fn new_name<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        IdCommand::Name(name.into())
    }

    /// Construct a new ID command with the given author.
    pub fn new_author<S>(author: S) -> Self
    where
        S: Into<String>,
    {
        IdCommand::Author(author.into())
    }
}

impl Display for IdCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content = match self {
            IdCommand::Name(name) => format!("name {name}"),
            IdCommand::Author(author) => format!("author {author}"),
        };

        write!(f, "id {content}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(IdCommand::new_name("Stockfish Engine"), "id name Stockfish Engine")]
    #[case(IdCommand::new_author("Stockfish Team"), "id author Stockfish Team")]
    fn format_id_cmd(#[case] input: IdCommand, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
