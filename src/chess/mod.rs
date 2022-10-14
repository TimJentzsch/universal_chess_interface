//! Several general chess constructs needed for UCI, such as moves.

mod file;
mod r#move;
mod move_line;
mod promotion_piece;
mod rank;
mod score;
mod square;

pub use file::*;
pub use move_line::*;
pub use promotion_piece::*;
pub use r#move::*;
pub use rank::*;
pub use score::*;
pub use square::*;

/// An error occured while parsing the given text.
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;
