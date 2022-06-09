 # Sudoku Solver
 This is a simple sudoku solver made in rust that uses backtracking to solve sudokus. Examples of the expected format of sudokus are shown in the file `sudoku.txt`, where each newline corresponds to a sudoku. The `.` characters correspond to unfilled numbers in the sudoku. New sudokus can be generated on this [website](https://qqwing.com/generate.html).

The project is built with cargo and a small test can be run with:
```rust
cargo run
```
This will output the sudokus in `sudoku.txt` as well as the solutions.
