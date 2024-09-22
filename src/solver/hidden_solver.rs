// use crate::{
//     auxiliar,
//     board::{Board, Cell},
// };
//
// fn hidden_group_solver() {}
//
// pub fn solve_hidden(board: &mut Board, quantity: usize) {
//     let groups = [Board::row, Board::col, Board::sqr];
//     let board_cells = board.clone();
//     for group in &groups {
//         for i in 0..9 {
//             let cells = group(&board_cells, i);
//             hidden_group_solver(board, cells, quantity);
//         }
//     }
// }
