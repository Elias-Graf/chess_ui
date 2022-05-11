use std::sync::{Arc, Mutex};

use chess_logic::Piece;
use eframe::{
    egui::{self, Widget},
    emath::pos2,
};

use super::piece_widget::PieceWidget;

pub fn promote_widget(
    selected_piece: &mut Option<chess_logic::Piece>,
    dynamic_texture_manager: Arc<Mutex<egui_extras::DynamicTextureManager>>,
) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        egui::Area::new("promote piece area")
            .fixed_pos(pos2(0.0, 0.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::window(&ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .add(PieceWidget::new(
                                Piece::Bishop,
                                chess_logic::Color::Black,
                                dynamic_texture_manager.clone(),
                            ))
                            .clicked()
                        {
                            *selected_piece = Some(Piece::Bishop);
                        }

                        if ui
                            .add(PieceWidget::new(
                                Piece::Knight,
                                chess_logic::Color::Black,
                                dynamic_texture_manager.clone(),
                            ))
                            .clicked()
                        {
                            *selected_piece = Some(Piece::Knight);
                        };

                        if ui
                            .add(PieceWidget::new(
                                Piece::Queen,
                                chess_logic::Color::Black,
                                dynamic_texture_manager.clone(),
                            ))
                            .clicked()
                        {
                            *selected_piece = Some(Piece::Queen);
                        }

                        if ui
                            .add(PieceWidget::new(
                                Piece::Rook,
                                chess_logic::Color::Black,
                                dynamic_texture_manager.clone(),
                            ))
                            .clicked()
                        {
                            *selected_piece = Some(Piece::Rook);
                        }
                    });
                })
            })
            .response
    }
}
