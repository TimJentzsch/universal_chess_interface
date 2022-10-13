use std::time::Duration;

use crate::chess::{Move, Score};

pub struct InfoCommand {
    /// Search depth in plies.
    depth: Option<usize>,

    /// Selective search depth in plies.
    ///
    /// If the engine sends `seldepth` there must also be a `depth` present in the same string.
    sel_depth: Option<usize>,

    /// The score from the engine's point of view.
    score: Option<Score>,

    /// The number of nodes that the engine searched.
    nodes: Option<usize>,

    /// The number of nodes that the engine searched per second.
    nodes_per_second: Option<usize>,

    /// The number of positions found in the endgame table bases.
    tb_hits: Option<usize>,

    /// The number of positions found in the shredder endgame databases.
    sb_hits: Option<usize>,

    /// The time searched.
    time: Option<Duration>,

    /// The best line found.
    pv: Option<Vec<Move>>,

    /// This for the multi pv mode.
    multi_pv: Option<usize>,

    /// The CPU load of the engine in permill.
    cpu_load_permill: Option<usize>,

    /// The hash table fill in permill.
    hash_full_permill: Option<usize>,

    /// Any string string which will be displayed by the engine,
    string: Option<String>,

    /// The given move is refuted by the given line.
    ///
    /// The engine should only send this if the option `UCI_ShowRefutations` is set to `true`.
    refutation: Option<(Move, Vec<Move>)>,

    /// The line the engine is currently calculating.
    ///
    /// The first paramater represents the CPU number.
    ///
    /// The engine should only send this if the option `UCI_ShowCurrLine` is set to `true`.
    curr_line: Option<(usize, Vec<Move>)>,
}
