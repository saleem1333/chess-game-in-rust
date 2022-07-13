use crate::board::pieces::PieceColor;
use crate::board::Board;
use crate::Position;

pub struct BoardPositionSelector<'a> {
    board: &'a Board,
    pos: Position,
    friendly_color: PieceColor,
    selected: Vec<Position>,
}

impl<'a> BoardPositionSelector<'a> {
    pub fn new(board: &'a Board, pos: Position, friendly_color: PieceColor) -> Self {
        Self {
            board,
            pos,
            friendly_color,
            selected: vec![],
        }
    }
}

impl BoardPositionSelector<'_> {
    pub fn build_positions(self) -> Vec<Position> {
        self.selected
    }
    pub fn vertical(&mut self) -> &mut Self {
        let i = self.pos.i;
        let j = self.pos.j;
        let up_file = self.handle_collisions_and_positions_validity(
            (i + 1..=7).map(|i| Position::new(i, j)).collect(),
        );
        let down_file = self.handle_collisions_and_positions_validity(
            (0..=i - 1).rev().map(|i| Position::new(i, j)).collect(),
        );

        self.selected.extend([up_file, down_file].iter().flatten());
        self
    }
    pub fn horizontal(&mut self) -> &mut Self {
        let i = self.pos.i;
        let j = self.pos.j;

        let right_row = self.handle_collisions_and_positions_validity(
            (j + 1..=7).map(|j| Position::new(i, j)).collect(),
        );
        let left_row = self.handle_collisions_and_positions_validity(
            (0..=j - 1).rev().map(|j| Position::new(i, j)).collect(),
        );

        self.selected.extend([right_row, left_row].iter().flatten());
        self
    }

    pub fn diagonal(&mut self) -> &mut Self {
        let i = self.pos.i;
        let j = self.pos.j;

        let mut temp = vec![];
        let mut c = 1;

        while i + c <= 7 && j + c <= 7 {
            temp.push(Position::new(i + c, j + c));
            c += 1;
        }
        self.selected
            .extend(self.handle_collisions_and_positions_validity(temp));

        let mut temp = vec![];
        c = 1;
        while i - c >= 0 && j - c >= 0 {
            temp.push(Position::new(i - c, j - c));
            c += 1;
        }
        self.selected
            .extend(self.handle_collisions_and_positions_validity(temp));

        let mut temp = vec![];
        c = 1;

        while i + c <= 7 && j - c >= 0 {
            temp.push(Position::new(i + c, j - c));
            c += 1;
        }

        self.selected
            .extend(self.handle_collisions_and_positions_validity(temp));

        let mut temp = vec![];
        c = 1;
        while i - c >= 0 && j + c <= 7 {
            temp.push(Position::new(i - c, j + c));
            c += 1;
        }
        self.selected
            .extend(self.handle_collisions_and_positions_validity(temp));
        self
    }

    pub fn custom(&mut self, positions: Vec<Position>) -> &mut Self {
        for pos in positions {
            // since it is not a range, handling collisions should be one position at a time
            self.selected
                .extend(self.handle_collisions_and_positions_validity(vec![pos]));
        }
        self
    }

    fn handle_collisions_and_positions_validity(&self, positions: Vec<Position>) -> Vec<Position> {
        let mut handled = Vec::with_capacity(positions.len());
        for pos in positions {
            if !Board::is_valid_position(pos) {
                continue;
            }
            let cell = self.board.look_up_cell(pos).unwrap();
            match cell {
                None => handled.push(pos),
                Some(piece) => {
                    if piece.color != self.friendly_color {
                        handled.push(pos)
                    }
                    break;
                }
            }
        }
        handled
    }
}
