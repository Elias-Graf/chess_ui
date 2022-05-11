// use std::fmt::{self, Display};

// use super::{Color, Piece, Player};

// #[derive(Clone)]
// pub struct Board {
//     board: Vec<Vec<PositionInfo>>,
//     height: i8,
//     opponent_color: Color,
//     width: i8,
//     you_color: Color,
// }

// impl Board {
//     pub fn new(you_color: Color, opponent_color: Color) -> Self {
//         let width = 8;
//         let height = 8;

//         let mut board = Self {
//             board: vec![vec![PositionInfo::None; width]; height],
//             height: height as i8,
//             opponent_color,
//             width: width as i8,
//             you_color,
//         };

//         // Standard chess formation:
//         board.set_piece(0, 0, Player::Opponent, Piece::Rook);
//         board.set_piece(1, 0, Player::Opponent, Piece::Knight);
//         board.set_piece(2, 0, Player::Opponent, Piece::Bishop);
//         board.set_piece(3, 0, Player::Opponent, Piece::Queen);
//         board.set_piece(4, 0, Player::Opponent, Piece::King);
//         board.set_piece(5, 0, Player::Opponent, Piece::Bishop);
//         board.set_piece(6, 0, Player::Opponent, Piece::Knight);
//         board.set_piece(7, 0, Player::Opponent, Piece::Rook);

//         board.set_piece(0, 1, Player::Opponent, Piece::Pawn);
//         board.set_piece(1, 1, Player::Opponent, Piece::Pawn);
//         board.set_piece(2, 1, Player::Opponent, Piece::Pawn);
//         board.set_piece(3, 1, Player::Opponent, Piece::Pawn);
//         board.set_piece(4, 1, Player::Opponent, Piece::Pawn);
//         board.set_piece(5, 1, Player::Opponent, Piece::Pawn);
//         board.set_piece(6, 1, Player::Opponent, Piece::Pawn);
//         board.set_piece(7, 1, Player::Opponent, Piece::Pawn);

//         board.set_piece(0, 7, Player::You, Piece::Rook);
//         board.set_piece(1, 7, Player::You, Piece::Knight);
//         board.set_piece(2, 7, Player::You, Piece::Bishop);
//         board.set_piece(3, 7, Player::You, Piece::Queen);
//         board.set_piece(4, 7, Player::You, Piece::King);
//         board.set_piece(5, 7, Player::You, Piece::Bishop);
//         board.set_piece(6, 7, Player::You, Piece::Knight);
//         board.set_piece(7, 7, Player::You, Piece::Rook);

//         board.set_piece(0, 6, Player::You, Piece::Pawn);
//         board.set_piece(1, 6, Player::You, Piece::Pawn);
//         board.set_piece(2, 6, Player::You, Piece::Pawn);
//         board.set_piece(3, 6, Player::You, Piece::Pawn);
//         board.set_piece(4, 6, Player::You, Piece::Pawn);
//         board.set_piece(5, 6, Player::You, Piece::Pawn);
//         board.set_piece(6, 6, Player::You, Piece::Pawn);
//         board.set_piece(7, 6, Player::You, Piece::Pawn);

//         board
//     }

//     pub fn width(&self) -> i8 {
//         return self.width;
//     }

//     pub fn height(&self) -> i8 {
//         return self.height;
//     }

//     /// Checks if a position is within the bounds of the board.
//     ///
//     /// The variable might safely be cased to [`usize`] after `true` was returned
//     /// from this function.
//     pub fn is_in_bounds(&self, x: i8, y: i8) -> bool {
//         x >= 0 && x < self.width && y >= 0 && y < self.height
//     }

//     pub fn display(&self) -> String {
//         const BG_BLACK: &str = "\u{001b}[40m";
//         const BG_WHITE: &str = "\u{001b}[47m";
//         const FG_BLACK: &str = "\u{001b}[30m";
//         const FG_PLAYER_BLACK: &str = "\u{001b}[38;5;54m";
//         const FG_PLAYER_WHITE: &str = "\u{001b}[38;5;207m";
//         const FG_MOVE: &str = "\u{001b}[91m";
//         const FG_WHITE: &str = "\u{001b}[37m";
//         const RESET: &str = "\u{001b}[0m";

//         let mut val = "  ".to_owned();

//         for letter in 'a'..'i' {
//             val.push_str(&format!(" {}  ", letter));
//         }

//         val.push('\n');

//         for y in 0..self.height {
//             // print the inverse, since the numbering of the squares starts at
//             // "the bottom", and the array starts "at the top".
//             val.push_str(&format!("{} ", self.height - y));

//             for x in 0..self.height {
//                 let piece_symbol: String;
//                 let mut bg_color: &str = BG_BLACK;
//                 let mut fg_color: &str = FG_WHITE;

//                 let is_even_row = y % 2 == 0;
//                 let is_even_column = x % 2 == 0;

//                 if is_even_row && is_even_column || !is_even_row && !is_even_column {
//                     bg_color = BG_WHITE;
//                     fg_color = FG_BLACK;
//                 }

//                 match self.get(x, y) {
//                     PositionInfo::Hit((piece, _)) => {
//                         fg_color = FG_MOVE;
//                         piece_symbol = Piece::get_symbol(piece);
//                     }
//                     PositionInfo::Piece((piece, player)) => {
//                         fg_color = match self.get_color_of_player(player) {
//                             Color::Black => FG_PLAYER_BLACK,
//                             Color::White => FG_PLAYER_WHITE,
//                         };
//                         piece_symbol = Piece::get_symbol(piece);
//                     }
//                     PositionInfo::Move => {
//                         piece_symbol = "..".to_owned();
//                     }
//                     PositionInfo::None => {
//                         piece_symbol = "  ".to_owned();
//                     }
//                 };

//                 val.push_str(&format!(
//                     "{}{} {} {}",
//                     fg_color, bg_color, piece_symbol, RESET
//                 ));
//             }

//             val.push_str(&format!(" {}", self.height - y));
//             val.push('\n');
//         }

//         val.push_str("  ");

//         for letter in 'a'..'i' {
//             val.push_str(&format!(" {}  ", letter));
//         }

//         val
//     }

//     pub fn set_piece(&mut self, x: i8, y: i8, player: Player, piece: Piece) {
//         assert!(
//             self.is_in_bounds(x, y),
//             "cannot set piece at out of bounds position ({}/{})",
//             x,
//             y
//         );

//         let pos = self.get(x, y);

//         if let PositionInfo::None = pos {
//             self.set(x as usize, y as usize, PositionInfo::Piece((piece, player)));
//             return;
//         }

//         panic!(
//             "pieces can only be set on empty positions, but position was {:?}",
//             pos
//         );
//     }

//     /// Returns `true` when a piece was hit and the piece should stop adding
//     /// additional moves (at least for the current direction).
//     pub fn set_for_piece_at_move_or_hit_at(
//         &mut self,
//         moving_piece_x: i8,
//         moving_piece_y: i8,
//         hit_piece_x: i8,
//         hit_piece_y: i8,
//     ) -> bool {
//         assert!(
//             self.is_in_bounds(moving_piece_x, moving_piece_y),
//             "moving piece is out of bounds ({}/{})",
//             moving_piece_x,
//             moving_piece_y
//         );
//         assert!(
//             self.is_in_bounds(hit_piece_x, hit_piece_y),
//             "hit piece is out of bounds ({}/{})",
//             hit_piece_x,
//             hit_piece_y
//         );

//         let moving_piece_pos_info = self.get(moving_piece_x, moving_piece_y);
//         let hit_pos = self.get(hit_piece_x, hit_piece_y);

//         match hit_pos {
//             PositionInfo::None => self.set(hit_piece_x as usize, hit_piece_y as usize, PositionInfo::Move),
//             PositionInfo::Piece((hit_piece, hit_player)) => {
//                 if let PositionInfo::Piece((_, moving_player)) = moving_piece_pos_info {
//                     if moving_player == hit_player {
//                         return true;
//                     }
//                 }
                
//                 let info = PositionInfo::Hit((*hit_piece, hit_player.clone())); 
//                 self.set(hit_piece_x as usize, hit_piece_y as usize, info);

//                 return true;
//             },
//             _ => panic!("moves or hits can only be set on positions that are empty or pieces are on, position was '{:?}'", hit_pos),
//         }

//         false
//     }

//     pub fn with_moves_for(&self, x: i8, y: i8) -> Self {
//         assert!(
//             self.is_in_bounds(x, y),
//             "cannot get board with moves for out of bounds position ({}/{})",
//             x,
//             y
//         );

//         let mut board = self.clone();

//         let (piece, player) = match self.get(x, y) {
//             PositionInfo::Piece(p) => p,
//             _ => return board,
//         };

//         Piece::add_moves_to_board(x, y, player, piece, &mut board);

//         board
//     }

//     pub fn get(&self, x: i8, y: i8) -> &PositionInfo {
//         assert!(
//             self.is_in_bounds(x, y),
//             "cannot get piece outside of board ({}/{})",
//             x,
//             y
//         );

//         &self.board[x as usize][y as usize]
//     }

//     pub fn get_color_of_player(&self, player: &Player) -> &Color {
//         match player {
//             Player::Opponent => &self.opponent_color,
//             Player::You => &self.you_color,
//         }
//     }

//     fn set(&mut self, x: usize, y: usize, info: PositionInfo) {
//         self.board[x][y] = info;
//     }
// }

// impl Display for Board {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.display())
//     }
// }

// #[derive(Clone, Debug)]
// pub enum PositionInfo {
//     Hit((Piece, Player)),
//     Move,
//     None,
//     Piece((Piece, Player)),
// }
