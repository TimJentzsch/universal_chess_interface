use super::{PromotionPiece, Square};

/// A move on the board, in long algebraic notation.
pub struct Move {
    source: Square,
    target: Square,
    promotion: Option<PromotionPiece>,
}
