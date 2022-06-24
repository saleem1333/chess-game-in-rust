use crate::board::pieces::Piece;
use crate::board::position::Position;

/// a cell on a board
#[derive(PartialEq, Clone, Debug)]
pub struct Cell {
    /// the piece on the cell
    pub piece: Option<Piece>,
    /// the position of the cell at the board
    pos: Position,
}

impl Cell {
    pub fn new(piece: Option<Piece>, pos: Position) -> Self {
        Self { piece, pos }
    }
    pub fn pos(&self) -> Position {
        self.pos
    }

    pub fn is_empty(&self) -> bool {
        self.piece.is_none()
    }
}
