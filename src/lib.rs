mod board;
mod visual;
mod aux;
mod solver;

use visual::print_board;
use board::Board;
use solver::sudoku_solver::solve;


pub fn run(){
    println!("starting...");
    let mut my_board = Board::create("379000014060010070080009005435007000090040020000800436900700080040080050850000249");
    print_board(&mut my_board, "beauty");

    solve(&mut my_board);
    print_board(&mut my_board, "beauty");

    println!("");
    visual::print_options(&my_board); 
    println!("");
}

#[allow(dead_code)]
#[cfg(test)]
mod tests {
    
    
}
