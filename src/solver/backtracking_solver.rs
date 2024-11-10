#![allow(dead_code, unused_imports)]

use crate::{
    auxiliar,
    board::{Board, Cell},
    solver::sudoku_solver::cell_update_options,
    visual,
};
// TODO - hacer algoritmo que detecte que el sudoku tiene soluciones multiples.
fn backtracking(board: &mut Board, cell_n: usize) -> bool {
    if cell_n > 80 {
        return true;
    }
    if board.cell(cell_n).original() {
        if backtracking(board, cell_n + 1) {
            return true;
        };
    } else {
        let basic_opts = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        board.mut_cell(cell_n).set_options(basic_opts);
        cell_update_options(board, cell_n);
        if board.cell(cell_n).options().iter().count() == 0 {
            return false;
        }
        let options = board.cell(cell_n).options().clone();
        for opt in options {
            board.mut_cell(cell_n).set_number(opt);
            if backtracking(board, cell_n + 1) {
                return true;
            }
            board.mut_cell(cell_n).set_number(0);
        }
    }
    false
}
/// solve the sudoku using the general backtracking function. If the sudoku is unsolvable returns
/// the original board.
pub fn solve_backtracking(board: &mut Board) {
    let original_board = board.clone();
    backtracking(board, 0);
    if !backtracking(board, 0) {
        *board = original_board;
    }
}

#[cfg(test)]
mod tests {
    use crate::{visual, Board};

    use super::solve_backtracking;
    #[test]
    fn test_hidden_pair() {
        // let unsolved_sudoku_raw: &str = "
        // 042 000 039
        // 090 000 000
        // 000 105 206
        //
        // 000 040 100
        // 005 006 000
        // 000 000 000
        //
        // 600 020 007
        // 408 700 390
        // 700 004 005
        // ";
        let unsolved_sudoku_raw: &str = "
        000 006 000
        000 070 000
        000 800 000

        500 020 000
        007 100 000
        030 009 000

        002 050 000
        060 300 000
        900 004 000
        ";
        let unsolved_sudoku = unsolved_sudoku_raw
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>();

        let solved_sudoku_raw: &str = "
        142 678 539
        596 432 871
        873 195 246

        967 243 158
        385 916 724
        214 857 963

        631 529 487
        458 761 392
        729 384 615
        ";
        let solved_sudoku_str = solved_sudoku_raw
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>();

        let mut my_board = Board::create(unsolved_sudoku.as_str());
        visual::print_board(&my_board, "beauty");

        solve_backtracking(&mut my_board);

        visual::print_board(&my_board, "beauty");
        let my_board_str: String = my_board
            .cells()
            .iter()
            .map(|x| x.number().to_string())
            .collect();
        assert_eq!(solved_sudoku_str, my_board_str);
    }
}
