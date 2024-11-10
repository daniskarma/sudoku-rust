mod auxiliar;
mod board;
mod solver;
mod visual;

use board::Board;
use solver::sudoku_solver::solve;
use visual::print_board;

#[allow(unused_variables)]
pub fn run() {
    let sudoku_easy_1 =
        "379000014060010070080009005435007000090040020000800436900700080040080050850000249";
    let sudoku_hard_1 =
        "501740008000000050098600400040961580050000010016854070005006730070000000900072805";

    let sudoku_veryhard_1 =
        "060089003039100006100730090020060017001000600570090020090024001600003480800610030";

    let sudoku_hiddentest =
        "009032000000700000162000000010020560000900000050000107000000403026009000005870000";

    println!("starting...");
    let mut my_board = Board::create(sudoku_veryhard_1);
    print_board(&my_board, "beauty");

    solve(&mut my_board);
    print_board(&my_board, "beauty");

    println!();
    visual::print_options(&my_board);
    println!();
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    #[test]
    fn test_test() {
        assert_eq!(1, 1);
    }
}
