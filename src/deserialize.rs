use std::error::Error;

use chess_logic::{board::PieceInstance, Board, Color, Piece, Player};
use yaml_rust::Yaml;

pub fn deserialize_game(ser_game: &Yaml) -> Result<Board, Box<dyn Error>> {
    deserialize_board(&ser_game["board"])
}

fn deserialize_board(ser_board: &Yaml) -> Result<Board, Box<dyn Error>> {
    let mut board = Board::new(Color::Black, Color::White);

    for (y, row) in ser_board.as_vec().unwrap().iter().enumerate() {
        for (x, entry) in row.as_vec().unwrap().iter().enumerate() {
            if !entry.is_null() {
                let player = deserialize_player(&entry["player"])?;
                let piece = deserialize_piece(&entry["piece"])?;
                
                let ins = PieceInstance::new(player, piece);

                board.set(x, y, Some(ins));
            }
        }
    }

    Ok(board)
}

fn deserialize_player(ser_player: &Yaml) -> Result<Player, Box<dyn Error>> {
    Ok(match ser_player.as_str().unwrap() {
        "You" => Player::You,
        "Opponent" => Player::Opponent,
        val => return Err(format!("failed to deserialize player with value '{}'", val).into()),
    })
}

fn deserialize_piece(ser_ins: &Yaml) -> Result<Piece, Box<dyn Error>> {
    Ok(match ser_ins.as_str().unwrap() {
        "Bishop" => Piece::Bishop,
        "King" => Piece::King,
        "Knight" => Piece::Knight,
        "Pawn" => Piece::Pawn,
        "Queen" => Piece::Queen,
        "Rook" => Piece::Rook,
        val => return Err(format!("failed to deserialize piece with value '{}'", val).into()),
    })
}
