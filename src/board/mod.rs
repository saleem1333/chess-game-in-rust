use crate::board::pieces::{Piece, PieceColor, PieceKind};
use crate::board::position::Position;
use crate::Turn;
use std::collections::HashMap;

pub mod cell;
pub mod pieces;
pub mod position;

pub const BOARD_SIZE: u8 = 64;

type Cell = Option<Piece>;

#[derive(Debug, Clone)]
pub struct Board {
    /// a board consists of cells
    cells: HashMap<Position, Cell>,
    /// to support en-passant rule
    passant_pos: Option<Position>,
    passant_tracker: u8,
}
impl Board {
    pub fn is_king_safe(&self, color: PieceColor) -> bool {
        self.is_safe_for(self.look_up_king_pos(color), color)
    }

    pub fn is_checkmate_for(&self, color: PieceColor) -> bool {
        let color = match color {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        };
        let mut cloned = self.clone();
        !self.is_king_safe(color) && self.cells
            .iter()
            .filter(|(_, cell)| cell.is_some())
            .map(|(pos, cell)| (pos, cell.as_ref().unwrap()))
            .filter(|(_, piece)| piece.color == color)
            .all(|(pos, _)| pieces::moves::get_legal_moves(*pos, &mut cloned, &color).is_empty())
    }
}

impl Board {
    pub fn new(cells: HashMap<Position, Option<Piece>>) -> Self {
        Self {
            cells,
            passant_pos: None,
            passant_tracker: 0,
        }
    }
    pub fn from_fen(s: &str) -> Self {
        let mut cells = HashMap::new();

        for i in 0..8_i8 {
            for j in 0..8_i8 {
                cells.insert(Position::new(i, j), Cell::None);
            }
        }
        let chars = s.chars();
        let mut row = 7_u8;
        let mut col = 0_u8;
        for c in chars {
            match c {
                c if c.is_ascii_alphabetic() => {
                    if col > 7 {
                        panic!("Max col reached");
                    }
                    let color = if c.is_ascii_lowercase() {
                        PieceColor::Black
                    } else {
                        PieceColor::White
                    };
                    let kind = match c.to_ascii_lowercase() {
                        'p' => PieceKind::Pawn,
                        'k' => PieceKind::King,
                        'q' => PieceKind::Queen,
                        'b' => PieceKind::Bishop,
                        'n' => PieceKind::Knight,
                        'r' => PieceKind::Rook,

                        _ => panic!("Invalid"),
                    };
                    cells.insert(
                        Position::new(row as i8, col as i8),
                        Cell::Some(Piece::new(kind, color)),
                    );
                    col += 1;
                }
                c if c.is_ascii_digit() => {
                    let digit = c.to_digit(10).unwrap() as u8;
                    if digit > 8 {
                        panic!("should be between 1 and 8")
                    }
                    col += digit;
                }
                '/' => {
                    if row < 1 {
                        panic!("Max row reached");
                    }
                    row -= 1;
                    col = 0;
                }
                _ => panic!("Invalid"),
            }
        }
        Self {
            cells,
            passant_pos: None,
            passant_tracker: 0,
        }
    }
    pub fn is_safe_for_king(&self, pos: Position, color: PieceColor) -> bool {
        self.look_up_king_pos(color) != pos
    }
}

pub enum BoardMoveError {
    NotYourTurn,
    NoPieceOnCell,
    /// an illegal move; eg: out of range
    Illegal,
}

impl Board {
    fn is_valid_position(pos: Position) -> bool {
        let range = 0..=7_i8;
        return range.contains(&pos.i()) && range.contains(&pos.j());
    }
}

impl Board {
    /// given a position; look up the corresponding cell in Self
    #[inline]
    pub fn look_up_cell(&self, pos: Position) -> Option<&Cell> {
        self.cells.get(&pos)
    }

    #[inline]
    fn look_up_mut_cell(&mut self, pos: Position) -> Option<&mut Cell> {
        self.cells.get_mut(&pos)
    }

    pub fn look_up_king_pos(&self, color: PieceColor) -> Position {
        *self.cells
            .iter()
            .find(|(_,cell)| {
                let piece = cell.as_ref();
                piece.map(|piece| piece.kind == PieceKind::King && piece.color == color).unwrap_or(false)
            })
            .expect("King doesn't exist on board! if you're calling get_legal_moves then you should probably use get_legal_moves_unchecked instead").0
    }

    pub fn is_safe_for(&self, pos: Position, color: PieceColor) -> bool {
        self.cells
            .iter()
            .filter(|(_, cell)| cell.is_some() && cell.as_ref().unwrap().color != color)
            .all(|(pos1, _)| !pieces::moves::get_legal_moves_unchecked(*pos1, self).contains(&pos))
    }

    pub fn move_piece(
        &mut self,
        turn: &mut Turn,
        fr: Position,
        to: Position,
    ) -> Result<(), BoardMoveError> {
        let fr_cell = self.look_up_cell(fr).expect("No such cell exists");

        if fr_cell.is_none() {
            return Err(BoardMoveError::NoPieceOnCell);
        }

        let piece = fr_cell.as_ref().unwrap();

        if *turn != piece.color {
            return Err(BoardMoveError::NotYourTurn);
        }

        return if pieces::moves::get_legal_moves(fr, self, turn).contains(&to) {
            // to see if self.passant_pos changes after move_force
            // if it actually changes to something else then we have
            // to set the self.passant_tracker back to 1
            let before_pos = self.passant_pos;
            pieces::moves::move_force(self, fr, to);
            *turn = match turn {
                Turn::White => Turn::Black,
                Turn::Black => Turn::White,
            };
            if self.passant_tracker == 1 && before_pos == self.passant_pos {
                self.passant_pos = None;
                self.passant_tracker = 0;
            }

            if self.passant_pos.is_some() {
                self.passant_tracker = 1;
            }
            Ok(())
        } else {
            Err(BoardMoveError::Illegal)
        };
    }
}
