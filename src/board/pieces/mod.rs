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

#[derive(Debug, PartialEq, Clone)]
pub enum PieceKind {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}
