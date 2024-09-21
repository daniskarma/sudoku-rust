#![allow(dead_code)]
use crate::board::{Board, Cell};

/// Returns list of options that have the required lenght (quantity)
fn isolate_quantiy_groups(region_opt: &Vec<Vec<u8>>, quantity: usize) -> Vec<Vec<u8>> {
    let mut group_list: Vec<Vec<u8>> = vec![];
    for opt in region_opt {
        if opt.len() > 1 && opt.len() <= quantity {
            group_list.push(opt.to_vec());
        }
    }
    group_list
}

/// Checks that a group only contains numbers that are within the numbers in group_compare
fn only_contains(group_original: &[u8], group_compare: &[u8]) -> bool {
    group_original.iter().all(|&x| group_compare.contains(&x))
}

/// Generates list of combined numbers of n size that will be possible candidates
fn generate_combinations(n: usize) -> Vec<Vec<u8>> {
    let mut results = Vec::new();
    let mut current = Vec::new();
    backtrack(&mut results, &mut current, n, 1);
    results
}

fn backtrack(results: &mut Vec<Vec<u8>>, current: &mut Vec<u8>, n: usize, start: u8) {
    if current.len() == n {
        results.push(current.clone());
        return;
    }

    for i in start..=9 {
        current.push(i);
        backtrack(results, current, n, i + 1);
        current.pop();
    }
}

/// Returns a list of options that repeat X times within the selected group. X being quantity.
/// The groups are supposedly filtered by isolate_quantity_groups() beforehand.
/// candidate_list will be a vec or the options Vec[u8] que podremos eliminar del resto de celdas
fn get_candidates(groups: Vec<Vec<u8>>, quantity: usize) -> Vec<Vec<u8>> {
    let mut candidate_list: Vec<Vec<u8>> = vec![];
    let combination_list: Vec<Vec<u8>> = generate_combinations(quantity);
    for combination in combination_list {
        let count = groups
            .iter()
            .filter(|&x| only_contains(x, &combination))
            .count();
        if count == quantity {
            candidate_list.push(combination.to_vec())
        }
    }
    candidate_list
}
// Solves naked groups on a sudoku region (file, row, sqr)
fn naked_group(board: &mut Board, region_cells: Vec<&Cell>, quantity: usize) {
    let mut cells_to_change: Vec<(usize, Vec<u8>)> = Vec::new();

    let region_opt = board.cells_to_opts(&region_cells);

    let isolated_groups = isolate_quantiy_groups(&region_opt, quantity);
    let candidates = get_candidates(isolated_groups, quantity);

    for cell in region_cells {
        if cell.original() {
            continue;
        }
        if cell.options().len() == 1 {
            continue;
        }
        for candidate in &candidates {
            if only_contains(cell.options(), candidate) {
                continue;
            }
            let mut new_opts = cell.options().clone();
            new_opts.retain(|&x| !candidate.contains(&x));
            cells_to_change.push((cell.id(), new_opts));
        }
    }
    for cell in cells_to_change {
        board.mut_cell(cell.0).set_options(cell.1);
    }
}

/// Generic function for naked pairs, triple or quadruple solving.
/// quantity parameter determine if its pairs, triple or quadruple.
pub fn solve_naked(board: &mut Board, quantity: usize) {
    let groups = [Board::row, Board::col, Board::sqr];
    let board_cells = board.clone();
    for group in &groups {
        for i in 0..9 {
            let cells = group(&board_cells, i);
            naked_group(board, cells, quantity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::isolate_quantiy_groups;
    use crate::{
        solver::naked_solver::{get_candidates, naked_group},
        Board,
    };

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

    #[test]
    fn test_naked_pair() {
        let mut board = create_sudoku1();
        board.mut_cell(0).set_options(vec![2, 3, 4, 5, 6, 7]);
        board.mut_cell(1).set_options(vec![9]);
        board.mut_cell(2).set_options(vec![1]);
        board.mut_cell(3).set_options(vec![3, 7]);
        board.mut_cell(4).set_options(vec![2, 4, 5, 6]);
        board.mut_cell(5).set_options(vec![4, 5]);
        board.mut_cell(6).set_options(vec![2, 5]);
        board.mut_cell(7).set_options(vec![2, 5]);
        board.mut_cell(8).set_options(vec![8]);
        let quantity = 2;
        let _board = board.clone();
        let my_group = _board.row_opt(0);
        let my_cells = _board.row(0);

        let isolated_groups = isolate_quantiy_groups(&my_group, quantity);
        assert_eq!(isolated_groups, vec![[3, 7], [4, 5], [2, 5], [2, 5]]);

        let candidates = get_candidates(isolated_groups, quantity);
        assert_eq!(candidates, vec![[2, 5]]);
        naked_group(&mut board, my_cells, quantity);
        let new_row_opts = board.row_opt(0);
        assert_eq!(
            new_row_opts,
            vec![
                vec![3, 4, 6, 7],
                vec![9],
                vec![1],
                vec![3, 7],
                vec![4, 6],
                vec![4],
                vec![2, 5],
                vec![2, 5],
                vec![8]
            ]
        )
    }

    #[test]
    fn test_naked_triplet() {
        // The triplet is 3,4,5 from cells 2, 6, 8
        let mut board = create_sudoku1();
        board.mut_cell(0).set_options(vec![7]);
        board.mut_cell(1).set_options(vec![8]);
        board.mut_cell(2).set_options(vec![3, 5]);
        board.mut_cell(3).set_options(vec![2, 4, 5, 6, 9]);
        board.mut_cell(4).set_options(vec![2, 5, 9]);
        board.mut_cell(5).set_options(vec![4, 5, 6, 9]);
        board.mut_cell(6).set_options(vec![3, 4, 5]);
        board.mut_cell(7).set_options(vec![1]);
        board.mut_cell(8).set_options(vec![3, 4, 5]);
        let quantity = 3;
        let _board = board.clone();
        let my_cells = _board.row(0);

        let test_row_final = vec![
            vec![7],
            vec![8],
            vec![3, 5],
            vec![2, 6, 9],
            vec![2, 9],
            vec![6, 9],
            vec![3, 4, 5],
            vec![1],
            vec![3, 4, 5],
        ];
        naked_group(&mut board, my_cells, quantity);
        let new_row_opts = board.row_opt(0);
        assert_eq!(new_row_opts, test_row_final);
    }
}
