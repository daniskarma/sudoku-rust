#![allow(dead_code)]
use crate::board::{Board, Cell};

/// Checks if the board is completed and valid. If so returns True.
pub fn validate_board(board: &Board) -> bool {
    for cell in board.cells() {
        if cell.number() == 0 {
            return false;
        }
    }
    let groups = [Board::row, Board::col, Board::sqr];
    for group in &groups {
        for i in 0..9 {
            let cells = group(board, i);
            if !validate_group(&cells) {
                return false;
            }
        }
    }
    true
}

/// Checks if one group (row, col, sqr) is valid. If so returns True.
fn validate_group(group: &Vec<&Cell>) -> bool {
    for number in 1..10 {
        let mut count = 0;
        for cell in group {
            if number == cell.number() {
                count += 1;
            }
        }
        if count != 1 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use crate::{solver::validate, Board};

    #[test]
    fn test_validate_unvalid() {
        let unsolved_sudoku: &str = "
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
        let sudoku = unsolved_sudoku
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>();

        let my_board = Board::create(sudoku.as_str());
        assert!(!validate::validate_board(&my_board));
    }
    #[test]
    fn test_validate_valid() {
        let solved_sudoku: &str = "
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

        let sudoku = solved_sudoku
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>();
        let my_board = Board::create(sudoku.as_str());
        assert!(validate::validate_board(&my_board));
    }
}
