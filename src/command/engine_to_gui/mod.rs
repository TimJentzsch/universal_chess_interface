//! Commands sent from the engine to the GUI.

mod best_move;
mod copy_protection;
mod id;
mod info;
mod option;
mod registration;

pub use self::best_move::BestMoveCommand;
pub use self::copy_protection::CopyProtectionCommand;
pub use self::id::IdCommand;
pub use self::info::InfoCommand;
pub use self::registration::RegistrationCommand;

/// A command sent from the engine to the GUI.
pub enum EngineToGuiCommand {
    /// Identify the engine to the GUI.
    Id(IdCommand),

    /// Must be sent after the `id` and optional options to tell the GUI that the engine has sent all infos and is ready in UCI mode.
    UciOk,

    /// This must be sent when the engine has received an `isready` command and has processed all input and is ready to accept new commands now.
    ReadyOk,

    /// the engine has stopped searching and found the move `<move>` best in this position.
    BestMove(BestMoveCommand),

    /// This is needed for copyprotected engines.
    ///
    /// After the `uciok` command the engine can tell the GUI, that it will check the copy protection now.
    /// This is done by `copyprotection checking`.
    ///
    ///  If the check is ok the engine should send `copyprotection ok`, otherwise `copyprotection error`.
    /// If there is an error the engine should not function properly but should not quit alone.
    /// If the engine reports copyprotection error the GUI should not use this engine and display an error message instead!
    CopyProtection(CopyProtectionCommand),

    /// This is needed for engines that need a username and/or a code to function with all features.
    Registration(RegistrationCommand),

    /// The engine wants to send information to the GUI.
    ///
    /// This should be done whenever one of the info has changed.
    Info(Box<InfoCommand>),

    /// This command tells the GUI which parameters can be changed in the engine.
    ///
    /// This should be sent once at engine startup after the `uci` and the `id` commands if any parameter can be changed in the engine.
    Option(),
}
