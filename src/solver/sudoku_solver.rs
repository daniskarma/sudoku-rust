use crate::{
    board::{Board, Cell},
    solver::naked_solver::{self},
    //visual::{self},
    solver::{
        hidden_solver::{self},
        pointing_solver,
    },
};

// Sudoku solving strategies at http://www.taupierbw.be/SudokuCoach/

/// Gives a Cell a set of options based on the basic rules of sudoku.
/// It will givive any number 1-9 that is not present in the col, row or sqr.
pub fn cell_update_options(board: &mut Board, n: usize) {
    {
        let cell = board.mut_cell(n);
        if cell.number() != 0 {
            cell.set_options(Vec::from([cell.number()]));
            return;
        }
    }

    let mut possibles = board.cell(n).options().clone();
    let row = board.row_num_fr_cell(n);
    let col = board.col_num_fr_cell(n);
    let sqr = board.sqr_num_fr_cell(n);

    possibles.retain(|&x| !row.contains(&x));
    possibles.retain(|&x| !col.contains(&x));
    possibles.retain(|&x| !sqr.contains(&x));

    board.mut_cell(n).set_options(possibles);
}
pub fn solve_update_options(board: &mut Board) {
    board_it_cells(board, cell_update_options);
}

/// If the cell have only one options it makes that option the number.
fn singles_alone(board: &mut Board, n: usize) {
    let cell: &mut Cell = board.mut_cell(n);
    let opts = cell.options();
    if opts.len() != 1 {
        return;
    }
    cell.set_number(opts[0]);
}
fn solve_singles_alone(board: &mut Board) {
    board_it_cells(board, singles_alone);
}

/// If some option only occurs in that cell once for its row, col or sqr, makes that option the number.
fn singles(board: &mut Board, n: usize) {
    let possibles = board.cell(n).options().clone();

    let row = board.row_opt_fr_cell(n).into_iter().flatten();
    let col = board.col_opt_fr_cell(n).into_iter().flatten();
    let sqr = board.sqr_opt_fr_cell(n).into_iter().flatten();
    let all_options = row.chain(col).chain(sqr);

    for option in possibles {
        let mut count = 0;
        count += all_options
            .clone()
            .fold(0, |acc, x| if x == option { acc + 1 } else { acc });
        if count == 1 {
            board.mut_cell(n).set_number(option);
        }
    }
}
fn solve_singles(board: &mut Board) {
    board_it_cells(board, singles);
}

/// Applies function over Board iterating over 81 cells.
fn board_it_cells(board: &mut Board, function: fn(&mut Board, usize)) {
    for n in 0..81 {
        if !board.cell(n).original() {
            function(board, n)
        }
    }
}

pub fn solve(board: &mut Board) {
    let mut passes = 0;
    loop {
        let prev_board = board.clone();
        solve_update_options(board);
        naked_solver::solve_naked(board, 2);
        naked_solver::solve_naked(board, 3);
        naked_solver::solve_naked(board, 4);
        hidden_solver::solve_hidden(board, 2);
        hidden_solver::solve_hidden(board, 3);
        hidden_solver::solve_hidden(board, 4);
        pointing_solver::solve_pointing(board, 2);
        pointing_solver::solve_pointing(board, 3);
        solve_singles_alone(board);
        solve_singles(board);
        passes += 1;

        // TODO - This is just for help finding bugs, it can be commented or deleted
        // visual::print_options(board);
        // visual::print_board(board, "beauty");

        if board.clone() == prev_board {
            break;
        }
        if passes > 20000 {
            break;
        } // TODO - make a reasonable approach to limit passes
    }
    println!("Terminated in {passes} passes.");
}
