mod board;
mod cell;

fn main() {
    let board = board::board::Board::new();
    board.print();
}
