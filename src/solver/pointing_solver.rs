#![allow(dead_code)]
use crate::board::Board;

fn count_on_same_line(opts_group: &[&Vec<u8>], quantity: usize, number: u8) -> bool {
    let count = opts_group
        .iter()
        .filter(|&options| options.contains(&number))
        .count();
    count == quantity
}

fn update_line(board: &mut Board, number: u8, line_number: usize, line_type: &str, sqr_num: usize) {
    let region_cells = if line_type == "row" {
        board.row(line_number)
    } else {
        board.col(line_number)
    };
    let mut cells_to_change: Vec<(usize, Vec<u8>)> = Vec::new();
    for cell in region_cells {
        if cell.sqr() == sqr_num {
            continue;
        };
        if cell.options().contains(&number) {
            let mut new_opts = cell.options().clone();
            new_opts.retain(|x| *x != number);
            cells_to_change.push((cell.id(), new_opts));
        }
    }
    for cell in cells_to_change {
        board.mut_cell(cell.0).set_options(cell.1);
    }
}

pub fn solve_pointing(board: &mut Board, quantity: usize) {
    for sqr_num in 0..9 {
        for number in 0..9 {
            let sqr_opts = board.sqr_opt(sqr_num);
            if sqr_opts.iter().filter(|opt| opt.contains(&number)).count() == quantity {
                let line_type = ["row", "col"];
                for t in line_type {
                    let mut line_opts = vec![];
                    for i in 0..3 {
                        for j in 0..3 {
                            if t == "row" {
                                line_opts.push(&sqr_opts[i * 3 + j]);
                            } else if t == "col" {
                                line_opts.push(&sqr_opts[j * 3 + i]);
                            }
                        }
                        if count_on_same_line(&line_opts, quantity, number) {
                            let line_abs_number = if t == "row" {
                                i + (sqr_num / 3) * 3
                            } else {
                                i + (sqr_num % 3) * 3
                            };
                            update_line(board, number, line_abs_number, t, sqr_num);
                        }
                        line_opts.clear();
                    }
                }
                // {
                //     // row check
                //     for row_number in 0..3 {
                //         let mut row_opts = vec![];
                //         for col_number in 0..3 {
                //             row_opts.push(&sqr_opts[row_number * 3 + col_number]);
                //         }
                //         if count_on_same_line(&row_opts, quantity, number) {
                //             let row_abs_number = row_number + (sqr_num / 3) * 3;
                //             update_row(board, row_abs_number, number, sqr_num);
                //             println!(
                //                 "row:{} number:{} sqr_num:{}",
                //                 row_number + (sqr_num / 3) * 3,
                //                 number,
                //                 sqr_num
                //             );
                //         };
                //     }
                // }
                // {
                //     // col check
                //     for col_number in 0..3 {
                //         let mut col_opts = vec![];
                //         for row_number in 0..3 {
                //             col_opts.push(&sqr_opts[row_number * 3 + col_number]);
                //         }
                //         if count_on_same_line(&col_opts, quantity, number) {
                //             println!(
                //                 "col:{} number:{} sqr_num:{}",
                //                 col_number + (sqr_num % 3) * 3,
                //                 number,
                //                 sqr_num
                //             );
                //         };
                //     }
                // }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::{pointing_solver::solve_pointing, sudoku_solver::solve_update_options};
    use crate::Board;

    fn create_sudoku1() -> Board {
        let sudoku_raw: &str = "
        000 000 000
        000 000 000
        000 000 000

        000 000 000
        000 000 000
        000 000 000

        000 000 000
        000 000 000
        000 000 000
        ";
        let sudoku = sudoku_raw
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>();
        let my_board = Board::create(sudoku.as_str());

        my_board
    }

    fn create_sudoku2() -> Board {
        let sudoku_raw: &str = "
        005 036 340
        324 007 016
        000 024 030

        100 000 300
        206 071 904
        009 000 001

        000 683 400
        402 710 003
        083 402 100
        ";
        let sudoku = sudoku_raw
            .chars()
            .filter(|&c| !c.is_whitespace())
            .collect::<String>();
        let my_board = Board::create(sudoku.as_str());

        my_board
    }
    #[test]
    fn test_pointing_pair() {
        let mut board = create_sudoku1();

        board.mut_cell(0).set_options(vec![5]);
        board.mut_cell(1).set_options(vec![8]);
        board.mut_cell(2).set_options(vec![6]);
        board.mut_cell(3).set_options(vec![2, 3, 7, 9]);
        board.mut_cell(4).set_options(vec![2, 3, 7, 9]);
        board.mut_cell(5).set_options(vec![2, 7, 9]);
        board.mut_cell(6).set_options(vec![4]);
        board.mut_cell(7).set_options(vec![1]);
        board.mut_cell(8).set_options(vec![2, 3]);

        board.mut_cell(9).set_options(vec![7]);
        board.mut_cell(10).set_options(vec![2, 9]);
        board.mut_cell(11).set_options(vec![2, 3, 9]);
        board.mut_cell(12).set_options(vec![8]);
        board.mut_cell(13).set_options(vec![1]);
        board.mut_cell(14).set_options(vec![4]);
        board.mut_cell(15).set_options(vec![2, 3, 9]);
        board.mut_cell(16).set_options(vec![5]);
        board.mut_cell(17).set_options(vec![6]);

        board.mut_cell(18).set_options(vec![3, 9]);
        board.mut_cell(19).set_options(vec![4]);
        board.mut_cell(20).set_options(vec![1]);
        board.mut_cell(21).set_options(vec![5]);
        board.mut_cell(22).set_options(vec![2, 3, 9]);
        board.mut_cell(23).set_options(vec![6]);
        board.mut_cell(24).set_options(vec![7]);
        board.mut_cell(25).set_options(vec![2, 3, 9]);
        board.mut_cell(26).set_options(vec![8]);

        solve_pointing(&mut board, 2);
        let new_row_opts = board.row_opt(1);
        assert_eq!(
            new_row_opts,
            vec![
                vec![7],
                vec![2, 9],
                vec![2, 3, 9],
                vec![8],
                vec![1],
                vec![4],
                vec![3, 9],
                vec![5],
                vec![6]
            ]
        );
    }

    #[test]
    fn test_pointing_triple() {
        let mut board = create_sudoku2();
        solve_update_options(&mut board);

        solve_pointing(&mut board, 3);
        let new_col_opts = board.col_opt(3);
        assert_eq!(
            new_col_opts,
            vec![
                vec![1, 8, 9],
                vec![5, 8, 9],
                vec![1, 5, 8, 9],
                vec![2, 5, 9],
                vec![3, 5],
                vec![2, 3, 5],
                vec![6],
                vec![7],
                vec![4]
            ]
        );
    }
}
