use crate::types::{
    Board,
    PieceTypes,
    Colors,
};

use crate::utils::{
    square_to_bit,
};

impl Board {
    pub fn fen_setup(&mut self, fen: &str) {
    let parts: Vec<&str> = fen.split_whitespace().collect();
    let board_part = parts[0];

    // Set active color
    let active_color = parts[1].chars().next().unwrap();
    self.side_to_move = match active_color {
        'w' => Colors::WHITE,
        'b' => Colors::BLACK,
        _ => panic!("Invalid active color"),
    };
    
    // TODO: parse castling rights
    let castling_rights = parts[2].to_string();
    self.castling_rights = 0;

    // TODO: parse en passant square
    let en_passant = if parts[3] == "-" {
        None
    } else {
        let file = parts[3].chars().next().unwrap() as u8 - b'a';
        let rank = parts[3].chars().nth(1).unwrap() as u8 - b'1';
        Some(square_to_bit((rank * 8 + file) as usize))
    };

    let ranks: Vec<&str> = board_part.split('/').collect();


    for (rank_index, rank) in ranks.iter().enumerate() {
        let mut file_index = 0;
        for c in rank.chars() {
            if c.is_digit(10) {
                file_index += c.to_digit(10).unwrap() as usize;
            } else {
                let square = (7 - rank_index) * 8 + file_index;
                let bit = square_to_bit(square);

                match c {
                    'P' => self.bitboards[Colors::WHITE][PieceTypes::PAWN] |= bit,
                    'N' => self.bitboards[Colors::WHITE][PieceTypes::KNIGHT] |= bit,
                    'B' => self.bitboards[Colors::WHITE][PieceTypes::BISHOP] |= bit,
                    'R' => self.bitboards[Colors::WHITE][PieceTypes::ROOK] |= bit,
                    'Q' => self.bitboards[Colors::WHITE][PieceTypes::QUEEN] |= bit,
                    'K' => self.bitboards[Colors::WHITE][PieceTypes::KING] |= bit,
                    'p' => self.bitboards[Colors::BLACK][PieceTypes::PAWN] |= bit,
                    'n' => self.bitboards[Colors::BLACK][PieceTypes::KNIGHT] |= bit,
                    'b' => self.bitboards[Colors::BLACK][PieceTypes::BISHOP] |= bit,
                    'r' => self.bitboards[Colors::BLACK][PieceTypes::ROOK] |= bit,
                    'q' => self.bitboards[Colors::BLACK][PieceTypes::QUEEN] |= bit,
                    'k' => self.bitboards[Colors::BLACK][PieceTypes::KING] |= bit,
                    _ => panic!("Invalid FEN"),
                }

                file_index += 1;
            }
        }
    }
    }
}