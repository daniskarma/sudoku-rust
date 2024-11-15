#![allow(dead_code, unused_imports)]
use crate::{
    auxiliar,
    board::{Board, Cell},
    solver::sudoku_solver::cell_update_options,
    visual,
};

fn fill_board(board: &mut Board, solution: &[u8]) {
    (0..81).for_each(|cell_n| {
        board.mut_cell(cell_n).set_number(solution[cell_n]);
    });
}

fn backtracking(
    board: &mut Board,
    cell_n: usize,
    solutions: &mut Vec<Vec<u8>>,
    max_sol: usize,
) -> bool {
    if cell_n > 80 {
        let sol_cells: Vec<u8> = board.cells().iter().map(|x| x.number()).collect();
        if !solutions.contains(&sol_cells) {
            solutions.push(sol_cells);
        }
        return solutions.len() > max_sol;
    }
    if board.cell(cell_n).original() {
        if backtracking(board, cell_n + 1, solutions, max_sol) {
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
            if backtracking(board, cell_n + 1, solutions, max_sol) {
                return true;
            }
            board.mut_cell(cell_n).set_number(0);
        }
    }
    false
}
//give back a vector of solutions or None if it excedes the maximum number of solutions
pub fn solutions_backtracking(board: &Board, max_sol: usize) -> Vec<Vec<u8>> {
    let mut working_board = board.clone();
    let mut solutions: Vec<Vec<u8>> = Vec::new();
    backtracking(&mut working_board, 0, &mut solutions, max_sol);
    if solutions.len() > max_sol {
        solutions.clear()
    }
    solutions
}

/// solve the sudoku using the general backtracking function. If the sudoku is unsolvable the
/// sudoku is unchanged
pub fn solve_backtracking(board: &mut Board) {
    let solutions = solutions_backtracking(board, 1);
    if solutions.len() == 1 {
        fill_board(board, &solutions[0]);
    }
}

#[cfg(test)]
mod tests {
    use crate::{solver::backtracking_solver::solutions_backtracking, visual, Board};

    use super::solve_backtracking;
    #[test]
    fn test_one_solution_solvable() {
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
    #[test]
    fn test_two_solutions_solvable() {
        let unsolved_sudoku_raw: &str = "
        042 000 030
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

        let my_board = Board::create(unsolved_sudoku.as_str());
        let solutions = solutions_backtracking(&my_board, 2);
        println!("{:?}", solutions);
        assert_eq!(solutions.len(), 2);
    }
    #[test]
    fn test_two_solutions_not_solvable() {
        let unsolved_sudoku_raw: &str = "
        042 000 030
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

        let mut my_board = Board::create(unsolved_sudoku.as_str());
        //visual::print_board(&my_board, "beauty");

        solve_backtracking(&mut my_board);

        //visual::print_board(&my_board, "beauty");
        let my_board_str: String = my_board
            .cells()
            .iter()
            .map(|x| x.number().to_string())
            .collect();
        assert_eq!(unsolved_sudoku, my_board_str);
    }
}
