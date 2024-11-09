#![allow(dead_code, unused_imports)]
use crate::{
    auxiliar,
    board::{Board, Cell},
};

// TODO continuar aqui
pub fn solve_backtracking(board: &mut Board) {}

#[cfg(test)]
mod tests {
    use crate::Board;

    use super::solve_backtracking;
    #[test]
    /// hidden pair [2, 7] in cells 6 and 8
    fn test_hidden_pair() {
        let unsolved_sudoku_raw: &str = "
        042 000 039
        090 000 000
        000 105 206

        000 040 100
        005 006 000
        000 000 000

        600 020 007
        408 700 390
        700 004 005
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

        solve_backtracking(&mut my_board);

        let my_board_str: String = my_board
            .cells()
            .iter()
            .map(|x| x.number().to_string())
            .collect();
        assert_eq!(solved_sudoku_str, my_board_str);
    }
}
