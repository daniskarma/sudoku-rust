#![allow(dead_code)]
use crate::{
    auxiliar,
    board::{Board, Cell},
};
/// Checks that a cell options numbers that are within the numbers in a combination
// BUG - el problema es que falta por detectar que todos los numeros de la combinacion
// aparezcan en mínimo una de las celdas
fn contains(cell_opts: &[u8], combination: &[u8]) -> bool {
    cell_opts.iter().any(|&opt| combination.contains(&opt))
}

/// Returns a list of options of size X that appears only on X cells within a region
fn get_candidates(region_opts: Vec<Vec<u8>>, quantity: usize) -> Vec<Vec<u8>> {
    let mut candidate_list: Vec<Vec<u8>> = vec![];
    let combination_list: Vec<Vec<u8>> = auxiliar::generate_combinations(quantity);
    for combination in combination_list {
        let mut count = region_opts
            .iter()
            .filter(|&cell_opts| contains(cell_opts, &combination))
            .count();
        if count == quantity {
            for number in &combination {
                if region_opts
                    .iter()
                    .filter(|&cell_opts| cell_opts.contains(&number))
                    .count()
                    <= 1
                {
                    count -= 1;
                }
            }
            if count == quantity {
                candidate_list.push(combination.to_vec())
            }
        }
    }
    candidate_list
}

/// Takes candidates and delete all options of a cell that are not the candidates.
fn hidden_group_solver(board: &mut Board, region_cells: Vec<&Cell>, quantity: usize) {
    let mut cells_to_change: Vec<(usize, Vec<u8>)> = Vec::new();

    let region_opt: Vec<Vec<u8>> = board.cells_to_opts(&region_cells);

    let candidates: Vec<Vec<u8>> = get_candidates(region_opt, quantity);

    for cell in region_cells {
        if cell.original() {
            continue;
        }
        if cell.options().len() == 1 {
            continue;
        }
        for candidate in &candidates {
            if !contains(cell.options(), candidate) {
                continue;
            }
            let mut new_opts = cell.options().clone();
            new_opts.retain(|&x| candidate.contains(&x));
            cells_to_change.push((cell.id(), new_opts));
        }
    }
    for cell in cells_to_change {
        board.mut_cell(cell.0).set_options(cell.1);
    }
}

pub fn solve_hidden(board: &mut Board, quantity: usize) {
    let groups = [Board::row, Board::col, Board::sqr];
    let board_cells = board.clone();
    for group in &groups {
        for i in 0..9 {
            let cells = group(&board_cells, i);
            hidden_group_solver(board, cells, quantity);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{solver::hidden_solver::hidden_group_solver, Board};

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
    /// hidden pair [2, 7] in cells 6 and 8
    fn test_hidden_pair() {
        let mut board = create_sudoku1();
        board.mut_cell(0).set_options(vec![6]);
        board.mut_cell(1).set_options(vec![3, 4, 8]);
        board.mut_cell(2).set_options(vec![5]);
        board.mut_cell(3).set_options(vec![3, 4, 8, 9]);
        board.mut_cell(4).set_options(vec![1]);
        board.mut_cell(5).set_options(vec![3, 4, 8, 9]);
        board.mut_cell(6).set_options(vec![2, 3, 4, 7, 9]);
        board.mut_cell(7).set_options(vec![3, 9]);
        board.mut_cell(8).set_options(vec![2, 7, 8, 9]);

        let quantity = 2;
        let _board = board.clone();
        let my_cells = _board.row(0);

        hidden_group_solver(&mut board, my_cells, quantity);
        let new_row_opts = board.row_opt(0);
        assert_eq!(
            new_row_opts,
            vec![
                vec![6],
                vec![3, 4, 8],
                vec![5],
                vec![3, 4, 8, 9],
                vec![1],
                vec![3, 4, 8, 9],
                vec![2, 7],
                vec![3, 9],
                vec![2, 7]
            ]
        )
    }

    #[test]
    /// hidden triplet [3, 4, 9] in cells 5, 7, 8
    fn test_hidden_triplet() {
        let mut board = create_sudoku1();
        board.mut_cell(0).set_options(vec![2, 5, 6, 8]);
        board.mut_cell(1).set_options(vec![2, 5, 8]);
        board.mut_cell(2).set_options(vec![5, 6]);
        board.mut_cell(3).set_options(vec![7]);
        board.mut_cell(4).set_options(vec![2, 5, 6]);
        board.mut_cell(5).set_options(vec![2, 3, 4, 5, 6, 9]);
        board.mut_cell(6).set_options(vec![3, 4, 5, 8]);
        board.mut_cell(7).set_options(vec![1]);
        board.mut_cell(8).set_options(vec![2, 4, 5, 6, 8, 9]);

        let quantity = 3;
        let _board = board.clone();
        let my_cells = _board.row(0);

        hidden_group_solver(&mut board, my_cells, quantity);
        let new_row_opts = board.row_opt(0);
        assert_eq!(
            new_row_opts,
            vec![
                vec![2, 5, 6, 8],
                vec![2, 5, 8],
                vec![5, 6],
                vec![7],
                vec![2, 5, 6],
                vec![3, 4, 9],
                vec![3, 4],
                vec![1],
                vec![4, 9]
            ]
        )
    }
}
