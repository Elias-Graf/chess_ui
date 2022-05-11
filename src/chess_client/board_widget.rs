use std::{
    fs,
    sync::{Arc, Mutex},
};

use chess_logic::{board::PieceInstance, Board, Color, InfoBoard, Piece, Player};
use eframe::{
    egui::{self, Image, Sense, Widget},
    emath::{pos2, Rect},
    epaint::{tessellator::Path, Color32, Rounding, Stroke},
};
use yaml_rust::{YamlEmitter, YamlLoader};

use crate::{deserialize_game, serialize_game};

use super::promote_widget::promote_widget;

const PIECE_SIZE: usize = 40;
const PIECE_TEXTURE_SIZE: egui_extras::dynamic_texture_manager::TextureSize =
    (PIECE_SIZE, PIECE_SIZE);
const PIECE_SIZE_VEC: egui::Vec2 =
    egui::vec2(PIECE_TEXTURE_SIZE.0 as f32, PIECE_TEXTURE_SIZE.1 as f32);

pub struct BoardWidget {
    board: chess_logic::Board,
    dynamic_texture_manager: Arc<Mutex<egui_extras::DynamicTextureManager>>,
}

impl BoardWidget {
    pub fn new(dynamic_texture_manager: Arc<Mutex<egui_extras::DynamicTextureManager>>) -> Self {
        fn ins(player: Player, piece: Piece) -> Option<PieceInstance> {
            let mut ins = PieceInstance::new(player, piece);
            ins.was_moved = true;

            Some(ins)
        }

        let board =
            if let Ok(file_content) = fs::read_to_string("C:/Users/Elias/Desktop/chess_test.yaml") {
                let ser_game = &YamlLoader::load_from_str(&file_content).unwrap()[0];

                deserialize_game(ser_game).unwrap()
            } else {
                chess_logic::Board::new_with_standard_formation(
                    chess_logic::Color::Black,
                    chess_logic::Color::White,
                )
            };

        // let mut board = Board::new(Color::Black, Color::White);
        // board.set(0, 7, ins(Player::You, Piece::King));
        // board.set(1, 6, ins(Player::You, Piece::Bishop));
        // board.set(7, 0, ins(Player::Opponent, Piece::King));
        // board.set(4, 4, ins(Player::Opponent, Piece::Pawn));

        Self {
            board,
            dynamic_texture_manager,
        }
    }

    fn paint_piece_at(
        &mut self,
        ui: &mut egui::Ui,
        piece: &chess_logic::Piece,
        piece_color: &chess_logic::Color,
        rect: &Rect,
    ) {
        let mut dynamic_texture_manager = self.dynamic_texture_manager.lock().unwrap();

        let texture_id =
            match (piece, piece_color) {
                (chess_logic::Piece::Bishop, chess_logic::Color::Black) => dynamic_texture_manager
                    .load_sized("src/assets/bishop_black.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::King, chess_logic::Color::Black) => dynamic_texture_manager
                    .load_sized("src/assets/king_black.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Knight, chess_logic::Color::Black) => dynamic_texture_manager
                    .load_sized("src/assets/knight_black.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Pawn, chess_logic::Color::Black) => dynamic_texture_manager
                    .load_sized("src/assets/pawn_black.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Queen, chess_logic::Color::Black) => dynamic_texture_manager
                    .load_sized("src/assets/queen_black.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Rook, chess_logic::Color::Black) => dynamic_texture_manager
                    .load_sized("src/assets/rook_black.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Bishop, chess_logic::Color::White) => dynamic_texture_manager
                    .load_sized("src/assets/bishop_white.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::King, chess_logic::Color::White) => dynamic_texture_manager
                    .load_sized("src/assets/king_white.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Knight, chess_logic::Color::White) => dynamic_texture_manager
                    .load_sized("src/assets/knight_white.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Pawn, chess_logic::Color::White) => dynamic_texture_manager
                    .load_sized("src/assets/pawn_white.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Queen, chess_logic::Color::White) => dynamic_texture_manager
                    .load_sized("src/assets/queen_white.svg", &PIECE_TEXTURE_SIZE),
                (chess_logic::Piece::Rook, chess_logic::Color::White) => dynamic_texture_manager
                    .load_sized("src/assets/rook_white.svg", &PIECE_TEXTURE_SIZE),
            };

        Image::new(texture_id, PIECE_SIZE_VEC).paint_at(ui, *rect);
    }

    fn paint_move_at(&self, ui: &mut egui::Ui, x: i8, y: i8) {
        ui.painter().circle(
            pos2(
                ((x as usize) * PIECE_SIZE) as f32 + PIECE_SIZE as f32 / 2.0,
                ((y as usize) * PIECE_SIZE) as f32 + PIECE_SIZE as f32 / 2.0,
            ),
            10.0,
            Color32::BLUE,
            Stroke::none(),
        )
    }

    fn paint_hit_at(&self, ui: &mut egui::Ui, x: i8, y: i8) {
        ui.painter().circle(
            pos2(
                ((x as usize) * PIECE_SIZE) as f32 + PIECE_SIZE as f32 / 2.0,
                ((y as usize) * PIECE_SIZE) as f32 + PIECE_SIZE as f32 / 2.0,
            ),
            10.0,
            Color32::RED,
            Stroke::none(),
        );
    }
}

impl Widget for &mut BoardWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let info_board = self.board.get_moves_of_selected();

        let promotion_in_progress =
            if let Some((promote_x, promote_y)) = self.board.get_promote_pos() {
                let mut selected_piece = None;

                ui.add(promote_widget(
                    &mut selected_piece,
                    self.dynamic_texture_manager.clone(),
                ));

                if let Some(selected_piece) = selected_piece {
                    self.board.promote_piece_to(selected_piece);
                }

                true
            } else {
                false
            };

        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                let bg_color = get_square_bg_color(y, x);
                let rect = get_square_rect_for_pos(x, y);

                ui.painter().rect_filled(rect, Rounding::none(), bg_color);

                match info_board.get(x, y) {
                    chess_logic::info_board::PosInfo::Move => self.paint_move_at(ui, x, y),
                    chess_logic::info_board::PosInfo::None => (),
                    chess_logic::info_board::PosInfo::Piece(instance) => {
                        let piece_color = self.board.get_color_of_player(&instance.player).clone();

                        self.paint_piece_at(ui, &instance.piece, &piece_color, &rect);
                    }
                    chess_logic::info_board::PosInfo::PieceHit(instance) => {
                        let piece_color = self.board.get_color_of_player(&instance.player).clone();

                        self.paint_hit_at(ui, x, y);
                        self.paint_piece_at(ui, &instance.piece, &piece_color, &rect);
                    }
                }

                let square = ui.allocate_rect(rect, Sense::click());

                if !promotion_in_progress && square.clicked() {
                    if let Some(_) = self.board.get_selected() {
                        let successfully_moved = self.board.move_selected_to(x, y);

                        if !successfully_moved {
                            self.board.update_selected(x, y);
                        }

                        let mut file_content = String::new();
                        let mut emitter = YamlEmitter::new(&mut file_content);

                        emitter.dump(&serialize_game(&self.board)).unwrap();

                        fs::write("C:/Users/Elias/Desktop/chess_test.yaml", file_content).unwrap();
                    } else {
                        self.board.update_selected(x, y);
                    }
                }
            }
        }

        ui.allocate_rect(
            Rect::from_two_pos(
                pos2(0 as f32, 0 as f32),
                pos2(
                    (self.board.width() as usize * PIECE_SIZE) as f32,
                    (self.board.height() as usize * PIECE_SIZE) as f32,
                ),
            ),
            Sense::click(),
        )
    }
}

fn get_square_bg_color(y: i8, x: i8) -> Color32 {
    const COLOR_BG_BLACK: Color32 = Color32::BROWN;
    const COLOR_BG_WHITE: Color32 = Color32::WHITE;

    let is_even_row = y % 2 == 0;
    let is_even_column = x % 2 == 0;

    match is_even_row && is_even_column || !is_even_row && !is_even_column {
        true => COLOR_BG_WHITE,
        false => COLOR_BG_BLACK,
    }
}

fn get_square_rect_for_pos(x: i8, y: i8) -> Rect {
    Rect::from_two_pos(
        pos2(
            ((x as usize) * PIECE_SIZE) as f32,
            ((y as usize) * PIECE_SIZE) as f32,
        ),
        pos2(
            ((x as usize) * PIECE_SIZE + PIECE_SIZE) as f32,
            ((y as usize) * PIECE_SIZE + PIECE_SIZE) as f32,
        ),
    )
}
