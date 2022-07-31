pub mod moves;

#[derive(Debug, PartialEq, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
    pub moved: bool,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        Self {
            kind,
            color,
            moved: false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum PieceKind {
    King = 5,
    Queen = 4,
    Rook = 3,
    Bishop = 2,
    Knight = 1,
    Pawn = 0,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}
