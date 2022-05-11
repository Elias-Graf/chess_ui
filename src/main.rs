use std::fs;

use chess::{chess_client, deserialize_game};
use chess_logic::{Board, Color};
use yaml_rust::{YamlEmitter, YamlLoader};

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "Chess",
        native_options,
        Box::new(|cc| Box::new(chess_client::ChessClient::new(cc))),
    );
}
