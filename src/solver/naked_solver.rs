#![allow(dead_code)]
use crate::board::{Board, Cell};

fn isolate_quantiy_groups (region_opt: &Vec<Vec<u8>>, quantity: usize) -> Vec<Vec<u8>>{
    let mut group_list:Vec<Vec<u8>> = vec![];
    for opt in region_opt {
        if opt.len() == quantity {
            group_list.push(opt.to_vec());
        }
    }
    group_list
}
fn get_candidates(groups: Vec<Vec<u8>>, quantity: usize) -> Vec<Vec<u8>>{
    let mut candidate_list:Vec<Vec<u8>> = vec![];
    for group in &groups {
        if candidate_list.iter().any(|x| x == group) {continue;}
        let count = groups.iter().filter(|&x| x == group).count();
        if count == quantity {
            candidate_list.push(group.to_vec())
        }
    }
    candidate_list
}

/// Generic function for naked pairs, triple or quadruple solving.
fn naked_group (board: &mut Board, region_opt: Vec<Vec<u8>>, region_cells: Vec<&Cell>, quantity: usize){
    let mut cells_to_change:Vec<(usize,Vec<u8>)> = Vec::new();

    let isolated_groups = isolate_quantiy_groups(&region_opt, quantity);
    let candidates = get_candidates(isolated_groups, quantity);

    for cell in region_cells {
        if cell.original() {continue;}
        if cell.options().len() == 1 {continue;}
        for candidate in &candidates {
            if cell.options() == candidate {continue;}
            let mut new_opts = cell.options().clone();
            new_opts.retain(|&x| !candidate.contains(&x));
            cells_to_change.push((cell.id(),new_opts));
        }
    }
    for cell in cells_to_change {
        board.mut_cell(cell.0).set_options(cell.1);
    }
}


pub fn solve_naked_pairs(board: &mut Board){
    let group = 2;
    for i in 0..9{
        let _board = board.clone();
        let row_cells = _board.row(i);
        let row_opt = _board.row_opt(i*9);
        naked_group(board, row_opt, row_cells, group);
    }
    for i in 0..9{
        let _board = board.clone();
        let col_cells = _board.col(i);
        let col_opt = _board.col_opt(i);
        naked_group(board, col_opt, col_cells, group);
    }
    for i in 0..9{
        let _board = board.clone();
        let sqr_cells = _board.sqr(i);
        let sqr_opt = _board.sqr_opt(i*3+27*(i/3));
        naked_group(board, sqr_opt, sqr_cells, group);
    }
    // TODO - los indices de sqr_opt son raros porque no se indexa igual sqr que sqr_opt
    // SOLUCIONARLO
}


#[cfg(test)]
mod tests {
    use crate::{solver::naked_solver::{get_candidates, naked_group}, Board};

    use super::isolate_quantiy_groups;


    fn create_sudoku1 () -> Board{
        let sudoku_raw: &str = "
        091 000 008
        000 000 000
        000 000 000

        000 000 000
        000 000 000
        000 000 000

        000 000 000
        000 000 000
        000 000 000
        ";
        let sudoku = sudoku_raw.chars().filter(|&c| !c.is_whitespace()).collect::<String>();
        let mut my_board = Board::create(sudoku.as_str());
        my_board.mut_cell(0).set_options(vec![2,3,4,5,6,7]);
        my_board.mut_cell(1).set_options(vec![9]);
        my_board.mut_cell(2).set_options(vec![1]);
        my_board.mut_cell(3).set_options(vec![3,7]);
        my_board.mut_cell(4).set_options(vec![2,4,5,6]);
        my_board.mut_cell(5).set_options(vec![4,5]);
        my_board.mut_cell(6).set_options(vec![2,5]);
        my_board.mut_cell(7).set_options(vec![2,5]);
        my_board.mut_cell(8).set_options(vec![8]);
        my_board
    }

    #[test]
    fn test_naked_pair_one() {
        let mut board = create_sudoku1();
        let quantity = 2;
        let _board = board.clone();
        let my_group = _board.row_opt(0);
        let my_cells = _board.row(0);

        let isolated_groups = isolate_quantiy_groups(&my_group, quantity);
        assert_eq!(isolated_groups, vec![[3,7],[4,5],[2,5],[2,5]]);

        let candidates = get_candidates(isolated_groups, quantity);
        assert_eq!(candidates, vec![[2,5]]);
        
        naked_group(&mut board, my_group, my_cells, quantity);
        let new_row_opts = board.row_opt(0);
        assert_eq!(new_row_opts, vec![vec![3,4,6,7],vec![9],vec![1],vec![3,7],vec![4,6],vec![4],vec![2,5],vec![2,5],vec![8]])
        
    }
} 
