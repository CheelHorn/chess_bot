mod types;
mod utils;
mod fen;

use self::types::{
    Board,
    PieceTypes,
    Colors,
};

use self::utils::{
    print_bitboard,
    print_board,
};



fn main() {
    println!("START!");

    let mut board = Board::new();


    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    
    board.fen_setup(&fen);


    print_bitboard(board.bitboards[Colors::WHITE][PieceTypes::ROOK]);

    println!("");

    print_bitboard(board.get_color_bitboard(Colors::WHITE));

    println!("");

    print_bitboard(board.get_color_bitboard(Colors::BLACK));

    println!("");

    print_board(board);
}

