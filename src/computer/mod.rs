use rand::Rng;

use crate::board::{
    pieces::{
        moves::{self, is_safe_to_move},
        PieceColor, PieceKind,
    },
    position::Position,
    Board,
};

type Move = (Position, Position);
pub struct ComputerEngine;
impl ComputerEngine {
    pub fn pick_move(&self, board: &Board, color: PieceColor) -> Move {
        let legal_moves: Vec<Move> = extract_legal_moves(board, color);
        let capturing_moves: Vec<Move> = get_capturing_moves(board, color, &legal_moves);

        if capturing_moves.is_empty() {
            let safe_moves = get_safe_moves(board, color, &legal_moves);

            if safe_moves.is_empty() {
                return pick_random_move(legal_moves);
            }
            return pick_random_move(safe_moves);
        }

        pick_random_move(capturing_moves)
    }
}

fn get_safe_moves(board: &Board, color: PieceColor, legal_moves: &Vec<Move>) -> Vec<Move> {
    let mut safe_moves = vec![];
    for (fr, to) in legal_moves {
        let fr_piece = board.look_up_cell(*fr).unwrap().as_ref().unwrap();
        if fr_piece.kind == PieceKind::King {
            continue;
        }
        if is_safe_to_move(*fr, *to, board, color) {
            safe_moves.push((*fr, *to))
        }
    }
    safe_moves
}

fn extract_legal_moves(board: &Board, color: PieceColor) -> Vec<Move> {
    let positions = board.get_all_pieces_pos_by_color(color);
    let all_legal_moves: Vec<Move> = positions
        .iter()
        .map(|pos| (*pos, moves::get_legal_moves(*pos, board, &color)))
        .filter(|(_, legal)| !legal.is_empty())
        .flat_map(|(fr, moves)| moves.iter().map(|to| (fr, *to)).collect::<Vec<Move>>())
        .collect();
    all_legal_moves
}
fn get_capturing_moves(board: &Board, color: PieceColor, legal_moves: &Vec<Move>) -> Vec<Move> {
    let mut res = vec![];
    for (fr, to) in legal_moves {
        let fr_piece = board.look_up_cell(*fr).unwrap().as_ref().unwrap();
        let to_piece = board.look_up_cell(*to).unwrap().as_ref();
        if let Some(to_piece) = to_piece {
            if to_piece.color != color
                && (fr_piece.kind == PieceKind::King
                || is_safe_to_move(*fr, *to, board, color)
                || fr_piece.kind <= to_piece.kind)
            {
                res.push((*fr, *to))
            }
        }
    }

    res
}

fn pick_random_move(moves: Vec<Move>) -> Move {
    let fr_rand = rand::thread_rng().gen_range(0..moves.len());
    moves[fr_rand]
}
