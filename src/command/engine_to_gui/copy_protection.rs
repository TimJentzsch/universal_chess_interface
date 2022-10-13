use std::fmt::Display;

/// A command for copy protected engines to communicate the status to the GUI.
pub enum CopyProtectionCommand {
    /// Tell the GUI that the copy protection is being checked now.
    Checking,

    /// The copy protection check has been successful.
    Ok,

    /// The copy protection check has failed.
    ///
    /// After this, the engine will not function properly anymore.
    Error,
}

impl Display for CopyProtectionCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            CopyProtectionCommand::Checking => "checking",
            CopyProtectionCommand::Ok => "ok",
            CopyProtectionCommand::Error => "error",
        };

        write!(f, "copyprotection {status}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(CopyProtectionCommand::Checking, "copyprotection checking")]
    #[case(CopyProtectionCommand::Ok, "copyprotection ok")]
    #[case(CopyProtectionCommand::Error, "copyprotection error")]
    fn format_best_move_cmd(#[case] input: CopyProtectionCommand, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
