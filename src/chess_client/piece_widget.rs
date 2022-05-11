use std::sync::{Arc, Mutex, MutexGuard};

use chess_logic::Piece;
use eframe::{
    egui::{Sense, Widget},
    emath::{vec2, Vec2},
};

const PIECE_TEXTURE_SIZE: (usize, usize) = (40, 40);
const PIECE_IMAGE_SIZE: Vec2 = vec2(PIECE_TEXTURE_SIZE.0 as f32, PIECE_TEXTURE_SIZE.1 as f32);

pub struct PieceWidget {
    color: chess_logic::Color,
    dynamic_texture_manager: Arc<Mutex<egui_extras::DynamicTextureManager>>,
    piece: chess_logic::Piece,
}

impl PieceWidget {
    pub fn new(
        piece: Piece,
        color: chess_logic::Color,
        dynamic_texture_manager: Arc<Mutex<egui_extras::DynamicTextureManager>>,
    ) -> Self {
        Self {
            color,
            dynamic_texture_manager,
            piece,
        }
    }
}

impl Widget for PieceWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let Self {
            color,
            dynamic_texture_manager,
            piece,
        } = self;

        let dynamic_texture_manager = dynamic_texture_manager.lock().unwrap();
        let texture_id = get_texture_id_of_piece(piece, color, dynamic_texture_manager);

        ui.image(texture_id, PIECE_IMAGE_SIZE)
            .interact(Sense::click())
    }
}

fn get_texture_id_of_piece(
    piece: Piece,
    color: chess_logic::Color,
    mut dynamic_texture_manager: MutexGuard<egui_extras::DynamicTextureManager>,
) -> eframe::epaint::TextureId {
    match (piece, color) {
        (Piece::Bishop, chess_logic::Color::Black) => {
            dynamic_texture_manager.load_sized("src/assets/bishop_black.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::King, chess_logic::Color::Black) => {
            dynamic_texture_manager.load_sized("src/assets/king_black.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Knight, chess_logic::Color::Black) => {
            dynamic_texture_manager.load_sized("src/assets/knight_black.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Pawn, chess_logic::Color::Black) => {
            dynamic_texture_manager.load_sized("src/assets/pawn_black.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Queen, chess_logic::Color::Black) => {
            dynamic_texture_manager.load_sized("src/assets/queen_black.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Rook, chess_logic::Color::Black) => {
            dynamic_texture_manager.load_sized("src/assets/rook_black.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Bishop, chess_logic::Color::White) => {
            dynamic_texture_manager.load_sized("src/assets/bishop_white.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::King, chess_logic::Color::White) => {
            dynamic_texture_manager.load_sized("src/assets/king_white.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Knight, chess_logic::Color::White) => {
            dynamic_texture_manager.load_sized("src/assets/knight_white.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Pawn, chess_logic::Color::White) => {
            dynamic_texture_manager.load_sized("src/assets/pawn_white.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Queen, chess_logic::Color::White) => {
            dynamic_texture_manager.load_sized("src/assets/queen_white.svg", &PIECE_TEXTURE_SIZE)
        }
        (Piece::Rook, chess_logic::Color::White) => {
            dynamic_texture_manager.load_sized("src/assets/rook_white.svg", &PIECE_TEXTURE_SIZE)
        }
    }
}
