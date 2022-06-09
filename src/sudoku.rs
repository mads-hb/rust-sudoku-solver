use std::fmt;
use std::fs;


pub struct SudokuBoard{
    board: Vec<usize>,
}


impl SudokuBoard {

    pub fn new_empty() -> Self {
        let vector: Vec<usize> =vec![0; 9*9];
        Self {board: vector}
    }

    pub fn from_string(sudoku_str: String) -> Self {
        let mut board = Self::new_empty();
        for (ix, c) in sudoku_str.chars().enumerate() {
            let val: usize;
            if c == '.' {
                val = 0;
            } else {
                val = usize::try_from(c.to_digit(10).expect("Expected to find an integer or '.'.")).unwrap();
            }
            board.set(ix/9, ix%9, val);
        }
        board
    }

    pub fn from_file(fname: &str) -> Self {
        let mut sudoku_str = fs::read_to_string(fname).expect("File could not be opened");
        sudoku_str = sudoku_str.trim().to_string();
        Self::from_string(sudoku_str)
    }

    fn get(&self, row: usize, col: usize) -> usize{
        self.board[row + col*9]
    }

    fn set(&mut self, row: usize, col: usize, value: usize) {
        self.board[row + col*9] = value;
    }

    fn is_valid_row(&self, row: usize, value: usize) -> bool {
        for i in 0..9{
            if self.get(row, i) == value{
                return false;
            }
        }
        true
    }
    
    fn is_valid_col(&self, col: usize, value: usize) -> bool {
        for i in 0..9{
            if self.get(i, col) == value{
                return false;
            }
        }
        true
    }

    fn is_valid_box(&self, row: usize, col: usize, value: usize) -> bool {
        let start_row = row - row%3;
        let start_col = col - col%3;
        for i in 0..3 {
            for j in 0..3 {
                if self.get(i+start_row, j + start_col) == value {
                    return false;
                }
            }
        }
        true
    }


    fn is_valid(&self, row: usize, col: usize, value: usize) -> bool {
        self.is_valid_row(row, value) && self.is_valid_col(col, value) && self.is_valid_box(row, col, value)
    }

    fn _solve(&mut self, mut row: usize, mut col: usize) -> bool {
        if row == 8 && col == 9 {
            return true;
        }
        if col == 9 {
            row += 1;
            col = 0;
        }

        if self.get(row, col) > 0 {
            return self._solve(row, col+1);
        }

        for value in 1..10 {
            if self.is_valid(row, col, value){
                self.set(row, col, value);
                if self._solve(row, col + 1){
                    return true;
                }
            }
            self.set(row, col, 0);
        }
        false
    }

    pub fn solve(&mut self){
        self._solve(0,0);
    }
}


impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            for j in 0..9 {
                write!(f, "{}\t", self.get(i, j)).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}
