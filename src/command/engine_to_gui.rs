/// A command sent from the engine to the GUI.
pub enum EngineToGuiCommand {
    Id(),
    UciOk,
    ReadyOk,
    BestMove(),
    CopyProtection(),
    Registration(),
    Info(),
    Option(),
}
