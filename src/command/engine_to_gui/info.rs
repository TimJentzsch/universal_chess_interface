use std::time::Duration;

use crate::chess::{Move, Score};

/// A command sending information from the engine to the GUI.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct InfoCommand {
    /// Search depth in plies.
    depth_plies: Option<usize>,

    /// Selective search depth in plies.
    ///
    /// If the engine sends `seldepth` there must also be a `depth` present in the same string.
    sel_depth_plies: Option<usize>,

    /// The score from the engine's point of view.
    score: Option<Score>,

    /// The number of nodes that the engine searched.
    node_count: Option<usize>,

    /// The number of nodes that the engine searched per second.
    nodes_per_second: Option<usize>,

    /// The number of positions found in the endgame table bases.
    endgame_tb_hits: Option<usize>,

    /// The number of positions found in the shredder endgame databases.
    shredder_tb_hits: Option<usize>,

    /// The time searched.
    time: Option<Duration>,

    /// The best line found.
    ///
    /// The first number represents the k-th best line.
    pv: Option<(usize, Vec<Move>)>,

    /// The CPU load of the engine in permill.
    cpu_load_permill: Option<usize>,

    /// The hash table fill in permill.
    hash_full_permill: Option<usize>,

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

    /// Any string string which will be displayed by the engine,
    string: Option<String>,
}

impl InfoCommand {
    /// Create a new, empty info command.
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Set the search depth (in plies).
    pub fn with_depth_plies(&mut self, depth_plies: usize) -> &mut Self {
        self.depth_plies = Some(depth_plies);
        self
    }

    /// Select the selective search depth (in plies).
    ///
    /// This should be set together with depth.
    pub fn with_sel_depth_plies(&mut self, sel_depth_plies: usize) -> &mut Self {
        self.sel_depth_plies = Some(sel_depth_plies);
        self
    }

    /// Set the score that the engine estimates for the current position.
    pub fn with_score(&mut self, score: Score) -> &mut Self {
        self.score = Some(score);
        self
    }

    /// Set the number of nodes that the engine has searched.
    pub fn with_node_count(&mut self, node_count: usize) -> &mut Self {
        self.node_count = Some(node_count);
        self
    }

    /// Set the number of nodes that the engine has searched per second.
    pub fn with_nodes_per_second(&mut self, nodes_per_second: usize) -> &mut Self {
        self.nodes_per_second = Some(nodes_per_second);
        self
    }

    /// Set the number of positions that were found in endgame table bases.
    pub fn with_endgame_tb_hits(&mut self, endgame_tb_hits: usize) -> &mut Self {
        self.endgame_tb_hits = Some(endgame_tb_hits);
        self
    }

    /// Set the number of positions that were found in the shredder endgame databases.
    pub fn with_shredder_tb_hits(&mut self, shredder_tb_hits: usize) -> &mut Self {
        self.shredder_tb_hits = Some(shredder_tb_hits);
        self
    }

    /// Set the time that the engine searched.
    ///
    /// Should be sent together with `pv`.
    pub fn with_time(&mut self, time: Duration) -> &mut Self {
        self.time = Some(time);
        self
    }

    /// Set the principal variation (PV).
    ///
    /// This represents the best line that the engine could find.
    ///
    /// If your engine supports multi-PV, use [`InfoCommand::with_multi_pv`] instead.
    pub fn with_pv(&mut self, principal_variation: Vec<Move>) -> &mut Self {
        self.pv = Some((1, principal_variation));
        self
    }

    /// Set the k-th best principal variation (PV).
    ///
    /// `k = 1` represents the best line, `k = 2` the second best, etc.
    pub fn with_multi_pv(&mut self, kth_best: usize, principal_variation: Vec<Move>) -> &mut Self {
        self.pv = Some((kth_best, principal_variation));
        self
    }

    /// Set the engine's CPU load, in permill.
    pub fn with_cpu_load_permill(&mut self, cpu_load_permill: usize) -> &mut Self {
        self.cpu_load_permill = Some(cpu_load_permill);
        self
    }

    /// Set the load of the hash table in permill.
    pub fn with_hash_full_permill(&mut self, hash_full_permill: usize) -> &mut Self {
        self.hash_full_permill = Some(hash_full_permill);
        self
    }

    /// The given move is refuted by the given line.
    ///
    /// The engine should only send this if the option `UCI_ShowRefutations` is set to `true`.
    pub fn with_refutation(&mut self, mv: Move, refutation: Vec<Move>) -> &mut Self {
        self.refutation = Some((mv, refutation));
        self
    }

    /// Set the currently calculated line for the first CPU.
    ///
    /// The engine should only send this if the option `UCI_ShowCurrLine` is set to `true`.
    pub fn with_curr_line(&mut self, line: Vec<Move>) -> &mut Self {
        self.curr_line = Some((1, line));
        self
    }

    /// Set the currently calculated line for a given CPU.
    ///
    /// The engine should only send this if the option `UCI_ShowCurrLine` is set to `true`.
    pub fn with_curr_line_for_cpu(&mut self, cpu_num: usize, line: Vec<Move>) -> &mut Self {
        self.curr_line = Some((cpu_num, line));
        self
    }

    /// Send any text as info.
    pub fn with_string<S>(&mut self, string: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.string = Some(string.into());
        self
    }
}
