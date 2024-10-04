use crate::utils::{
    square_to_bit,
};

use std::fmt;

pub type Bitboard = u64;
pub type Square = usize;
pub type Color = usize;
pub type PieceType = usize;

pub struct Squares;
impl Squares {
    pub const COUNT:Square = 64;
}

// Color type
pub struct Colors;
impl Colors {
    pub const WHITE:Color = 0;
    pub const BLACK:Color = 1;
    pub const COUNT:Color = 2;
}

// Piece Type type
// All chess piece types
pub struct PieceTypes;
impl PieceTypes {
    pub const PAWN:PieceType = 0;
    pub const KNIGHT:PieceType = 1;
    pub const BISHOP:PieceType = 2;
    pub const ROOK:PieceType = 3;
    pub const QUEEN:PieceType = 4;
    pub const KING:PieceType = 5;
    pub const COUNT:PieceType = 6;
}

// Piece type
// Each piece has a piece type and a color
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self.piece_type {
            PieceTypes::PAWN => "P",
            PieceTypes::KNIGHT => "N",
            PieceTypes::BISHOP => "B",
            PieceTypes::ROOK => "R",
            PieceTypes::QUEEN => "Q",
            PieceTypes::KING => "K",
            _ => "Unknown",
        };

        let formatted_piece = match self.color {
            Colors::WHITE => symbol.to_uppercase(),
            Colors::BLACK => symbol.to_lowercase(),
            _ => "Unknown".to_string(),
        };
        write!(f, "{}", formatted_piece)
    }
}

// Board type
pub struct Board {
    pub bitboards: [[Bitboard; PieceTypes::COUNT]; Colors::COUNT],
    pub side_to_move: Color,
    pub castling_rights: u8,
    pub en_passant_square: u8,
    pub halfmove_clock: u8,
    pub fullmove_number: u8,
}

impl Board {
    pub fn new() -> Board {
        Board {
            bitboards: [[0; PieceTypes::COUNT]; Colors::COUNT],
            side_to_move: Colors::WHITE,
            castling_rights: 0,
            en_passant_square: 0,
            halfmove_clock: 0,
            fullmove_number: 1,
        }
    }

    pub fn put_piece(&mut self, piece: &Piece, square: Square) {
        let mask = square_to_bit(square);
        self.bitboards[piece.color][piece.piece_type] |= mask;
    }

    pub fn remove_piece(&mut self, piece: &Piece, square: Square) {
        let mask = square_to_bit(square);
        self.bitboards[piece.color][piece.piece_type] &= !mask;
    }

    pub fn move_piece(&mut self, piece: Piece, from: Square, to: Square) {
        self.remove_piece(&piece, from);
        self.put_piece(&piece, to);
    }

    pub fn get_piece(&self, square: Square) -> Option<Piece> {
        for color in 0..Colors::COUNT {
            for piece_type in 0..PieceTypes::COUNT {
                let mask = square_to_bit(square);
                if (self.bitboards[color][piece_type] & mask) != 0 {
                    return Some(Piece {piece_type, color});
                }
            }
        }
        None
    }

    pub fn get_color_bitboard(&self, color: Color) -> Bitboard {
        let mut result = 0;
        for piece_type in 0..PieceTypes::COUNT {
            result |= self.bitboards[color][piece_type];
        }
        result
    }
}