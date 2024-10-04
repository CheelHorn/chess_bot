use crate::types::{
    Bitboard,
    Board,
};

pub fn print_bitboard(bitboard:Bitboard) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            let mask = 1 << index;
            print!("{}", if (bitboard & mask) != 0 { "1 " } else { "0 " });
        }
        println!();
    }
}

pub fn print_board(board: Board) {
    for rank in (0..8).rev() {
        for file in 0..8 {
            let index = rank * 8 + file;
            let piece = board.get_piece(index);
            match piece {
                Some(piece) => print!("{} ", piece),
                None => print!("0 "),
            }

        }
        println!();
    }
}

pub fn square_to_bit(square: usize) -> u64 {
    1 << square
}

