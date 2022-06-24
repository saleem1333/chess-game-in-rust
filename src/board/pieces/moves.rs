use crate::board::pieces::PieceKind::Pawn;
use crate::board::pieces::{Piece, PieceKind};
use crate::board::position::selector::BoardPositionSelector;
use crate::board::position::Position;
use crate::board::{Board, Cell};
use crate::Turn;

pub fn get_legal_moves(piece_pos: Position, board: &mut Board, turn: &Turn) -> Vec<Position> {
    let unchecked_moves = get_legal_moves_unchecked(piece_pos, board);
    let mut legal_moves = Vec::with_capacity(unchecked_moves.len());

    let original_piece_cell = board.look_up_mut_cell(piece_pos).unwrap();
    let mut mover = original_piece_cell.take();
    let color = mover.as_ref().unwrap().color;
    if *turn == color {
        for possible_move in unchecked_moves {
            // no need to worry about the look_up_mut_cell's efficiency since it is O(1)
            // it utilizes a hashmap under the hood

            let move_kind = get_move_kind(
                mover.as_ref().unwrap(),
                board.look_up_cell(possible_move).unwrap().as_ref(),
                piece_pos,
                possible_move,
            );

            if let MoveKind::Castle = move_kind {
                let padding = if piece_pos.j() < possible_move.j() {
                    -1
                } else {
                    1
                };
                // if king is in check or there is an enemy piece cutting off the way for the king
                if !board.is_safe_for(piece_pos, color)
                    || !legal_moves
                        .contains(&(possible_move.i(), possible_move.j() + padding).into())
                {
                    continue;
                }
            }

            let cell = board.look_up_mut_cell(possible_move).unwrap();
            let mut target_cell: Cell = cell.take(); // saves the piece at the target position

            *cell = mover; // moves the original piece to the target position

            if let MoveKind::EnPassant = move_kind {
                target_cell = board
                    .look_up_mut_cell(Position::new(piece_pos.i(), possible_move.j()))
                    .unwrap()
                    .take();

                board.passant_pos = None;
            }
            if board.is_king_safe(color) {
                legal_moves.push(possible_move);
            }
            let cell = board.look_up_mut_cell(possible_move).unwrap(); // because of the borrow checker rules. we need to look up the cell again
            mover = cell.take();

            if let MoveKind::EnPassant = move_kind {
                let cell = board
                    .look_up_mut_cell(Position::new(piece_pos.i(), possible_move.j()))
                    .unwrap();
                *cell = target_cell;
                board.passant_pos = Some(Position::new(piece_pos.i(), possible_move.j()));
            } else {
                *cell = target_cell; // restores the piece that was at that position
            }
        }
    }

    let original_piece_cell = board.look_up_mut_cell(piece_pos).unwrap();
    *original_piece_cell = mover; // restores the original position of the piece
    legal_moves
}

/// returns the legal moves without considering king's safety
pub fn get_legal_moves_unchecked(piece_pos: Position, board: &Board) -> Vec<Position> {
    let piece = board.look_up_cell(piece_pos).unwrap().as_ref().unwrap();
    let color = piece.color;
    let mut selector = BoardPositionSelector::new(board, piece_pos, piece.color);
    let mut vec = Vec::<Position>::new();
    let i = piece_pos.i();
    let j = piece_pos.j();
    return match piece.kind {
        PieceKind::King => {
            selector.custom(vec![
                Position::new(i, j + 1),
                Position::new(i, j - 1),
                Position::new(i + 1, j),
                Position::new(i - 1, j),
                Position::new(i + 1, j + 1),
                Position::new(i + 1, j - 1),
                Position::new(i - 1, j + 1),
                Position::new(i - 1, j - 1),
            ]);
            if !piece.moved {
                let king_side_rook_pos = Position::new(i, 7);
                let queen_side_rook_pos = Position::new(i, 0);

                let king_side_rook = board.look_up_cell(king_side_rook_pos).unwrap();
                let queen_side_rook = board.look_up_cell(queen_side_rook_pos).unwrap();
                let is_king_side_path_free = [Position::new(i, j + 1), Position::new(i, j + 2)]
                    .into_iter()
                    .map(|pos| board.look_up_cell(pos))
                    .all(|cell| cell.unwrap().is_none());
                let is_queen_side_path_free = [
                    Position::new(i, j - 1),
                    Position::new(i, j - 2),
                    Position::new(i, j - 3),
                ]
                .into_iter()
                .map(|pos| board.look_up_cell(pos))
                .all(|cell| cell.unwrap().is_none());

                if let Some(rook) = king_side_rook {
                    if !rook.moved && is_king_side_path_free {
                        selector.custom(vec![Position::new(i, j + 2)]);
                    }
                }

                if let Some(rook) = queen_side_rook {
                    if !rook.moved && is_queen_side_path_free {
                        selector.custom(vec![Position::new(i, j - 2)]);
                    }
                }
            }

            selector.build_positions()
        }
        PieceKind::Queen => {
            selector.vertical().horizontal().diagonal();

            selector.build_positions()
        }
        PieceKind::Knight => {
            selector.custom(vec![
                Position::new(i + 2, j + 1),
                Position::new(i + 2, j - 1),
                Position::new(i - 2, j + 1),
                Position::new(i - 2, j - 1),
                Position::new(i + 1, j + 2),
                Position::new(i + 1, j - 2),
                Position::new(i - 1, j + 2),
                Position::new(i - 1, j - 2),
            ]);

            selector.build_positions()
        }
        PieceKind::Bishop => {
            selector.diagonal();
            selector.build_positions()
        }
        PieceKind::Rook => {
            selector.vertical().horizontal();
            selector.build_positions()
        }
        PieceKind::Pawn => {
            let piece_pos_adaptive = piece_pos.adaptive(color); // CRUCIAL STEP
            let i = piece_pos_adaptive.i();
            let j = piece_pos_adaptive.j();
            let upper = Position::new(i + 1, j).adaptive(color);
            let upper_right = Position::new(i + 1, j + 1).adaptive(color);
            let upper_left = Position::new(i + 1, j - 1).adaptive(color);

            let upper_cell = board.look_up_cell(upper);
            let upper_right_cell = board.look_up_cell(upper_right);
            let upper_left_cell = board.look_up_cell(upper_left);

            if upper_cell.is_some() && upper_cell.unwrap().is_none() {
                vec.push(upper);
                let initial_row = 1; // since the positions are adaptive; the initial row for the pawns always start at 1

                let pos: Position = Position::new(i + 2, j).adaptive(color);
                let upper_upper_cell = board.look_up_cell(pos);
                if !piece.moved && i == initial_row && upper_upper_cell.unwrap().is_none() {
                    vec.push(pos);
                }
            }

            if upper_right_cell.is_some()
                && upper_right_cell.unwrap().is_some()
                && upper_right_cell.unwrap().as_ref().unwrap().color != color
            {
                vec.push(upper_right)
            }

            if upper_left_cell.is_some()
                && upper_left_cell.unwrap().is_some()
                && upper_left_cell.unwrap().as_ref().unwrap().color != color
            {
                vec.push(upper_left)
            }

            // enpasssant

            let passant_pos = board.passant_pos;
            if let Some(passant_pos) = passant_pos {

                let pawn_passant = board.look_up_cell(passant_pos).unwrap().as_ref().unwrap();

                let is_same_row = piece_pos.i() == passant_pos.i();

                if piece_pos != passant_pos && is_same_row {
                    let right_passant = piece_pos.j() + 1 == passant_pos.j();
                    let left_passant = piece_pos.j() - 1 == passant_pos.j();
                    if pawn_passant.color != color {
                        if right_passant {
                            vec.push(
                                Position::new(piece_pos_adaptive.i() + 1, piece_pos.j() + 1)
                                    .adaptive(color),
                            );
                        } else if left_passant {
                            vec.push(
                                Position::new(piece_pos_adaptive.i() + 1, piece_pos.j() - 1)
                                    .adaptive(color),
                            );
                        }
                    }
                }
            }

            vec
        }
    };
}

// assume moving from "fr" to "to" is legal
pub(crate) fn move_force(board: &mut Board, fr: Position, to: Position) {
    match get_move_kind(
        board.look_up_cell(fr).unwrap().as_ref().unwrap(),
        board.look_up_cell(to).unwrap().as_ref(),
        fr,
        to,
    ) {
        MoveKind::Castle => {
            let padding = if fr.j() > to.j() { 1 } else { -1 };
            let king_new_pos: Position = (to.i(), to.j()).into();
            let rook_new_pos: Position = (fr.i(), king_new_pos.j() + padding).into();

            {
                let fr_cell = board.look_up_mut_cell(fr).unwrap();
                let temp = fr_cell.take();

                let to_cell = board.look_up_mut_cell(king_new_pos).unwrap();
                *to_cell = temp;
                to_cell.as_mut().unwrap().moved = true;
            }

            {
                let rook_pos = Position::new(fr.i(), if padding > 0 { 0 } else { 7 });
                let fr_cell = board.look_up_mut_cell(rook_pos).unwrap();
                let temp = fr_cell.take();
                let to_cell = board.look_up_mut_cell(rook_new_pos).unwrap();
                *to_cell = temp;
                to_cell.as_mut().unwrap().moved = true;
            }
        }
        MoveKind::Promote => {
            let fr_cell = board.look_up_mut_cell(fr).unwrap().take();
            let color = fr_cell.unwrap().color;
            let to_cell = board.look_up_mut_cell(to).unwrap();
            *to_cell = Some(Piece::new(PieceKind::Queen, color))
        }
        MoveKind::Regular => {
            let fr_cell = board.look_up_mut_cell(fr).unwrap();
            let temp = fr_cell.take();
            let to_cell = board.look_up_mut_cell(to).unwrap();
            *to_cell = temp;
            to_cell.as_mut().unwrap().moved = true;

            if to_cell.as_ref().unwrap().kind == Pawn && fr.i().abs_diff(to.i()) == 2 {
                board.passant_pos = Some(to);
            }
        }
        MoveKind::EnPassant => {
            let fr_cell = board.look_up_mut_cell(fr).unwrap();
            let temp = fr_cell.take();
            let to_cell = board.look_up_mut_cell(to).unwrap();
            *to_cell = temp;
            to_cell.as_mut().unwrap().moved = true;

            let _ = board
                .look_up_mut_cell(Position::new(fr.i(), to.j()))
                .unwrap()
                .take();
        }
    }
}

enum MoveKind {
    Promote,
    EnPassant,
    Castle,
    Regular,
}
// ASSUME: moving is legal
fn get_move_kind(
    piece_fr: &Piece,
    piece_to: Option<&Piece>,
    fr: Position,
    to: Position,
) -> MoveKind {
    if piece_fr.kind == PieceKind::King && fr.j().abs_diff(to.j()) > 1 {
        return MoveKind::Castle;
    }
    if piece_fr.kind == Pawn && to.adaptive(piece_fr.color).i() == 7 {
        return MoveKind::Promote;
    }

    if piece_fr.kind == PieceKind::Pawn && piece_to.is_none() && fr.j() != to.j() {
        return MoveKind::EnPassant;
    }
    return MoveKind::Regular;
}
