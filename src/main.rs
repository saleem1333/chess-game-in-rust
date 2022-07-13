use chess::board::Board;
use chess::Turn;
use chess_ui::ChessUI;
use iced::{pure::Application, window::Settings};

fn main() {
    let settings = ChessSettings::new(
        Board::default(),
        Turn::White,
        (Player::Computer, Player::Computer),
    );

    let game = ChessGame::new(settings);
    game.run();
}

#[allow(dead_code)]
enum Player {
    Human,
    Computer,
}

pub struct ChessGame {
    settings: ChessSettings,
}

impl ChessGame {
    pub fn new(settings: ChessSettings) -> Self {
        Self { settings }
    }
    pub fn run(self) {
        ChessUI::run(iced::Settings {
            window: Settings {
                size: (800, 800),
                resizable: false,
                ..Default::default()
            },
            flags: self.settings,
            id: None,
            default_font: None,
            default_text_size: 20,
            text_multithreading: false,
            antialiasing: false,
            exit_on_close_request: true,
            try_opengles_first: false,
        })
        .unwrap()
    }
}

pub struct ChessSettings {
    board: Board,
    turn: Turn,
    players: (Player, Player),
}

impl ChessSettings {
    fn new(board: Board, turn: Turn, players: (Player, Player)) -> Self {
        Self {
            board: board,
            turn: turn,
            players,
        }
    }
}

mod chess_ui {

    use chess::board::pieces::{moves, PieceColor, PieceKind};
    use chess::board::position::Position;
    use chess::board::{Board, Cell};
    use chess::Turn;
    use iced::button::StyleSheet;
    use iced::pure::widget::{button, Button, Column, Row};
    use iced::pure::{Application, Element};
    use iced::{Background, Color, Length, Svg};
    use rand::Rng;

    use crate::{ChessSettings, Player};

    pub struct ChessUI {
        board: Board,
        turn: Turn,
        players: (Player, Player),
        current_selected: Option<Position>,
        legal_moves: Vec<Position>,
        king_state: KingState,
    }

    impl ChessUI {
        fn new(board: Board, turn: Turn, players: (Player, Player)) -> Self {
            Self {
                board: board,
                turn: turn,
                players,
                current_selected: None,
                legal_moves: Vec::new(),
                // TODO: should depend on the board setup
                king_state: KingState::Safe,
            }
        }
        fn current_player(&self) -> &Player {
            match self.turn {
                PieceColor::White => &self.players.0,
                PieceColor::Black => &self.players.1,
            }
        }
    }
    struct SquareStyleSheet {
        color: Color,
    }

    enum KingState {
        Safe,
        Check(Position),
        Checkmate(Position),
    }

    impl StyleSheet for SquareStyleSheet {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(self.color)),
                border_width: 0.78,
                border_color: Color::BLACK,
                ..Default::default()
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum CellClickedEvent {
        Clicked(Position),
    }
    impl Application for ChessUI {
        type Executor = iced::executor::Default;
        type Message = CellClickedEvent;
        type Flags = ChessSettings;

        fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
            (
                ChessUI::new(flags.board, flags.turn, flags.players),
                iced::Command::none(),
            )
        }

        fn title(&self) -> String {
            "Chess".to_string()
        }

        fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
            if let KingState::Checkmate(_) = self.king_state {
                return iced::Command::none();
            }
            match message {
                CellClickedEvent::Clicked(pos) => match self.current_player() {
                    Player::Human => {
                        if self.current_selected.is_none() {
                            let piece = self.board.look_up_cell(pos).unwrap();

                            if piece.is_some() {
                                self.current_selected = Some(pos);
                                self.legal_moves =
                                    moves::get_legal_moves(pos, &mut self.board, &self.turn);
                            } else {
                                self.legal_moves.clear();
                                self.current_selected = None;
                            }

                            return iced::Command::none();
                        }
                        match self.board.move_piece(
                            &mut self.turn,
                            self.current_selected.unwrap(),
                            pos,
                        ) {
                            Ok(_) => {
                                self.current_selected = None;
                                self.legal_moves.clear();
                            }
                            Err(err) => match err {
                                chess::board::BoardMoveError::NotYourTurn
                                | chess::board::BoardMoveError::Illegal => {
                                    let piece = self.board.look_up_cell(pos).unwrap();

                                    if piece.is_some() {
                                        self.current_selected = Some(pos);
                                        self.legal_moves = moves::get_legal_moves(
                                            pos,
                                            &mut self.board,
                                            &self.turn,
                                        );
                                    } else {
                                        self.legal_moves.clear();
                                        self.current_selected = None;
                                    }
                                }
                                _ => {}
                            },
                        }
                    }
                    Player::Computer => {
                        let positions = self.board.get_all_pieces_pos_by_color(self.turn);
                        let all_legal_moves: Vec<(Position, Vec<Position>)> = positions
                            .iter()
                            .map(|pos| {
                                (
                                    *pos,
                                    moves::get_legal_moves(*pos, &mut self.board, &self.turn),
                                )
                            })
                            .filter(|(_, legal)| !legal.is_empty())
                            .collect();
                        let (to, fr) = pick_legal_move(all_legal_moves);

                        self.board.move_piece(&mut self.turn, fr, to).unwrap();
                    }
                },
            }

            let king_pos = self.board.look_up_king_pos(self.turn);
            if !self.board.is_safe_for(king_pos, self.turn) {
                self.king_state = KingState::Check(king_pos);
            } else {
                self.king_state = KingState::Safe;
            }

            if self.board.is_checkmate_for(match self.turn {
                PieceColor::White => PieceColor::Black,
                PieceColor::Black => PieceColor::White,
            }) {
                self.king_state = KingState::Checkmate(king_pos);
                println!("CHECKMATED!");
            }

            iced::Command::none()
        }

        fn view(&self) -> Element<'_, Self::Message> {
            let mut column = Column::new();

            for i in (0..8).rev() {
                let mut row = Row::new();
                for j in 0..8 {
                    let mut color = if (i + j) % 2 == 0 {
                        Color::from_rgba8(74, 101, 71, 20.0)
                    } else {
                        Color::WHITE
                    };

                    let piece_str = get_piece_str(self.board.look_up_cell((i, j).into()).unwrap());

                    let path = format!("{}/src/icons/{piece_str}.svg", env!("CARGO_MANIFEST_DIR"));

                    let svg = Svg::from_path(path);

                    if self.legal_moves.contains(&(i, j).into()) {
                        color = Color::from_rgba8(226, 203, 128, 35.0);
                    }

                    if let KingState::Check(pos) = self.king_state {
                        if pos == Position::new(i, j) {
                            color = Color::from_rgb8(204, 48, 76);
                        }
                    }

                    if let KingState::Checkmate(pos) = self.king_state {
                        if pos == Position::new(i, j) {
                            color = Color::from_rgb8(230, 48, 76);
                        }
                    }
                    if let Some(pos) = self.current_selected {
                        if pos == Position::new(i, j) {
                            color = Color::from_rgba8(195, 228, 235, 10.0);
                        }
                    }

                    let square = Button::new(svg)
                        .on_press(CellClickedEvent::Clicked((i, j).into()))
                        .padding(11)
                        .width(Length::Units(100))
                        .height(Length::Units(100))
                        .style(SquareStyleSheet { color: color });
                    row = row.push(square);
                }
                column = column.push(row);
            }

            column.into()
        }
    }

    fn get_piece_str(cell: &Cell) -> String {
        let mut piece_str = "".to_string();

        if let Some(piece) = cell {
            if piece.color == PieceColor::White {
                piece_str.push_str("white_");
            } else {
                piece_str.push_str("black_");
            }
            piece_str.push_str(match piece.kind {
                PieceKind::King => "king",
                PieceKind::Queen => "queen",
                PieceKind::Knight => "knight",
                PieceKind::Bishop => "bishop",
                PieceKind::Rook => "rook",
                PieceKind::Pawn => "pawn",
            });
        }
        piece_str
    }

    fn pick_legal_move(legal_moves: Vec<(Position, Vec<Position>)>) -> (Position, Position) {
        let fr_rand = rand::thread_rng().gen_range(0..legal_moves.len());
        let fr = &legal_moves[fr_rand];
        let to_rand = rand::thread_rng().gen_range(0..fr.1.len());
        let to = fr.1[to_rand];
        let fr = fr.0;
        (to, fr)
    }
}
