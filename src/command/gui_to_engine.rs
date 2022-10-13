/// A command sent from the GUI to the engine.
pub enum GuiToEngineCommand {
    Uci,
    Debug(bool),
    IsReady,
    SetOption(),
    Register(),
    UciNewGame(),
    Position(),
    Go(),
    Stop,
    PonderHit,
    Quit,
}
