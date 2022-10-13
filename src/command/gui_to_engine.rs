//! Commands sent from the GUI to the engine.

/// A command sent from the GUI to the engine.
pub enum GuiToEngineCommand {
    /// Tell engine to use the Universal Chess Interface (UCI).
    ///
    /// This will be sent once as a first command after program boot to tell the engine to switch to UCI mode.
    ///
    /// After receiving the `uci` command the engine must identify itself with the `id` command
    /// and send the `option` commands to tell the GUI which engine settings the engine supports if any.
    ///
    /// After that the engine should send `uciok` to acknowledge the UCI mode.
    /// If no `uciok` is sent within a certain time period, the engine task will be killed by the GUI.
    Uci,

    /// Switch the debug mode of the engine on and off.
    ///
    /// In debug mode the engine should send additional infos to the GUI, e.g. with the `info string` command, to help debugging, e.g. the commands that the engine has received etc.
    ///
    /// This mode should be switched off by default and this command can be sent any time, also when the engine is thinking.
    Debug(bool),

    /// This is used to synchronize the engine with the GUI.
    ///
    /// When the GUI has sent a command or multiple commands that can take some time to complete, this command can be used to wait for the engine to be ready again or to ping the engine to find out if it is still alive.
    /// E.g. this should be sent after setting the path to the tablebases as this can take some time.
    ///
    /// This command is also required once before the engine is asked to do any search to wait for the engine to finish initializing.
    ///
    /// This command must always be answered with `readyok` and can be sent also when the engine is calculating in which case the engine should also immediately answer with readyok without stopping the search.
    IsReady,

    /// This is sent to the engine when the user wants to change the internal parameters of the engine.
    SetOption(),

    /// This is the command to try to register an engine or to tell the engine that registration will be done later.
    ///
    /// This command should always be sent if the engine has sent `registration error` at program startup.
    Register(),

    /// This is sent to the engine when the next search (started with `position` and `go`) will be from a different game.
    ///
    /// This can be a new game the engine should play or a new game it should analyse but also the next position from a testsuite with positions only.
    ///
    /// sIf the GUI hasn't sent a `ucinewgame` before the first position command, the engine shouldn't expect any further `ucinewgame` commands as the GUI is probably not supporting the `ucinewgame` command.
    /// So the engine should not rely on this command even though all new GUIs should support it.
    ///
    /// As the engine's reaction to `ucinewgame` can take some time the GUI should always send `isready` after `ucinewgame` to wait for the engine to finish its operation.
    UciNewGame(),

    /// Set up the position described in `<fenstring>` on the internal board and play the moves on the internal chess board.
    ///
    /// If the game was played from the start position the string `startpos` will be sent.
    ///
    /// Note: no "new" command is needed.
    /// However, if this position is from a different game than the last position sent to the engine, the GUI should have sent a `ucinewgame` inbetween.
    Position(),

    /// Start calculating on the current position set up with the `position` command.
    Go(),

    /// Stop calculating as soon as possible.
    ///
    /// Don't forget the `bestmove` and possibly the `ponder` token when finishing the search.
    Stop,

    /// The user has played the expected move.
    ///
    /// This will be sent if the engine was told to ponder on the same move the user has played.
    /// The engine should continue searching but switch from pondering to normal search.
    PonderHit,

    /// Quit the program as soon as possible.
    Quit,
}
