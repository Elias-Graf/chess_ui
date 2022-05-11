use std::sync::{Arc, Mutex};

use eframe::egui::{self, Widget};

mod board_widget;
mod piece_widget;
mod promote_widget;

pub struct ChessClient {
    board: board_widget::BoardWidget,
    dynamic_texture_manager: Arc<Mutex<egui_extras::DynamicTextureManager>>,
}

impl ChessClient {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::default());

        let dynamic_texture_manager =
            Arc::new(Mutex::new(egui_extras::DynamicTextureManager::new(
                cc.egui_ctx.tex_manager(),
                Box::new(egui_extras::dynamic_texture_manager::bytes_loader::FsBytesLoader),
            )));

        Self {
            board: board_widget::BoardWidget::new(dynamic_texture_manager.clone()),
            dynamic_texture_manager,
        }
    }
}

impl eframe::App for ChessClient {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.button("hello world");
            // ui.add(PromoteWidget::new(self.dynamic_texture_manager.clone()));
            self.board.ui(ui);
        });
    }
}
