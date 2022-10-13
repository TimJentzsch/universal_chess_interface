use std::fmt::Display;

/// A command for engines that need registration to communicate the status to the GUI.
pub enum RegistrationCommand {
    /// Tell the GUI that the registration is being checked now.
    Checking,

    /// The registration check has been successful.
    Ok,

    /// The registration check has failed.
    ///
    /// Not all engine features might work anymore.
    Error,
}

impl Display for RegistrationCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            RegistrationCommand::Checking => "checking",
            RegistrationCommand::Ok => "ok",
            RegistrationCommand::Error => "error",
        };

        write!(f, "registration {status}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(RegistrationCommand::Checking, "registration checking")]
    #[case(RegistrationCommand::Ok, "registration ok")]
    #[case(RegistrationCommand::Error, "registration error")]
    fn format_best_move_cmd(#[case] input: RegistrationCommand, #[case] expected: String) {
        let actual = format!("{input}");
        assert_eq!(actual, expected);
    }
}
