use chess_logic::{board::PieceInstance, Board};
use linked_hash_map::LinkedHashMap;
use yaml_rust::Yaml;

pub fn serialize_game(board: &Board) -> Yaml {
    let mut ser_game = LinkedHashMap::new();

    ser_game.insert(Yaml::String("board".to_owned()), serialize_board(board));

    Yaml::Hash(ser_game)
}

fn serialize_board(board: &Board) -> Yaml {
    let mut ser_rows = Vec::new();

    for y in 0..8 {
        let mut ser_row = Vec::new();

        for x in 0..8 {
            let ser_piece = match board.get(x, y) {
                Some(ins) => serialize_piece_ins(ins),
                None => Yaml::Null,
            };

            ser_row.push(ser_piece);
        }

        ser_rows.push(Yaml::Array(ser_row));
    }

    Yaml::Array(ser_rows)
}

fn serialize_piece_ins(ins: &PieceInstance) -> Yaml {
    let mut ser = LinkedHashMap::new();

    ser.insert(
        Yaml::String("piece".to_owned()),
        Yaml::String(format!("{:?}", ins.piece)),
    );
    ser.insert(
        Yaml::String("player".to_owned()),
        Yaml::String(format!("{:?}", ins.player)),
    );
    ser.insert(
        Yaml::String("was_moved".to_owned()),
        Yaml::Boolean(ins.was_moved),
    );
    ser.insert(
        Yaml::String("is_eligible_for_en_passant".to_owned()),
        Yaml::Boolean(ins.is_eligible_for_en_passant),
    );

    Yaml::Hash(ser)
}
