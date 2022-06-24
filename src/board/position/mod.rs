pub mod selector;

use crate::board::pieces::PieceColor;

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
pub struct Position {
    i: i8,
    j: i8,
}

trait PositionNotation<'a> {
    fn from_notation(notation: &str) -> Self;
    fn to_notation(self) -> &'a str;
}

impl Position {
    const MAX_I: i8 = 7;
    const MAX_J: i8 = 7;

    pub fn max_i(j: i8) -> Position {
        Self {
            i: Position::MAX_I,
            j,
        }
    }

    pub fn max_j(i: i8) -> Position {
        Self {
            i,
            j: Position::MAX_J,
        }
    }
    pub fn i(&self) -> i8 {
        self.i
    }

    pub fn j(&self) -> i8 {
        self.j
    }

    pub fn new(i: i8, j: i8) -> Self {
        Self { i, j }
    }

    pub fn adaptive(self, color: PieceColor) -> Self {
        match color {
            PieceColor::White => (self.i, self.j).into(),
            PieceColor::Black => (Position::MAX_I - self.i, self.j).into(),
        }
    }
}

impl From<(i8, i8)> for Position {
    fn from((x, y): (i8, i8)) -> Self {
        Position::new(x, y)
    }
}

impl<'a> PositionNotation<'a> for Position {
    fn from_notation(_notation: &str) -> Self {
        todo!()
    }

    fn to_notation(self) -> &'a str {
        todo!()
    }
}
