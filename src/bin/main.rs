use std::time::Instant;
use sudoku::sudoku::SudokuBoard;
use std::fs;


fn main() {
    let mut sudoku_str = fs::read_to_string("sudoku.txt").unwrap();
    sudoku_str = sudoku_str.trim().to_string();
    
    for line in sudoku_str.lines() {
        let mut board = SudokuBoard::from_string(line.to_string());
        println!("{}", board);
        let now = Instant::now();
        board.solve();
        println!("{}", board);
        println!("Solved in {}ms.", now.elapsed().as_millis());
    }
}
