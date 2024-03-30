#![allow(dead_code)]

use crate::board::Board;

pub fn print_board(board: &mut Board, mode: &str){
    match mode{
        "plain" => print_plain(board),
        "beauty" => print_beauty(board),
        _ => (),
    }    
}

fn print_plain(board: &mut Board){
    for i in 0..9{
        for j in 0..9 {
            let n = i*9+j;
            let current_cell_number = board.cell(n).number();
            print!("{current_cell_number} ");
            if j == 8 {println!{""}};
        }
    }
}

fn print_beauty(board: &mut Board){
    println!("+-------+-------+-------+");
    for i in 0..9{
        print!("| ");
        for j in 0..9 {
            let n = i*9+j;
            let current_cell_number = board.cell(n).number();
            let printed_number:String;
            if current_cell_number == 0 {
                printed_number = String::from("·")
            } else{
                printed_number = board.cell(n).number().to_string()
            }


            if board.cell(n).original() {
                print!("\x1b[33m{printed_number}\x1b[0m ");
            } else {
                print!("{printed_number} ");
            }
            if j == 2 || j == 5 {print!("| ")}
            if j == 8 {println!{"|"}};
        }
        if i == 2 || i== 5 {
            println!("+-------+-------+-------+")
        }
    }
    println!("+-------+-------+-------+");
}

pub fn print_options(board: &Board) {
    let opt_size = maximum_opt_len(board);
    for i in 0..9 {
        for j in 0..9 {
            let n = i*9+j;
            let current_options = board.cell(n).options();
            print!("{current_options:?}");
            let space = opt_size*3+1 - (current_options.len()*3); 
            
            print!("{}"," ".repeat(space));
        }
        println!();
    }
    println!();
}

fn maximum_opt_len(board: &Board) -> usize {
    let mut maximun = 0;
    for i in 0..81 {
        let size = board.cell(i).options().len();
        if size > maximun{
            maximun = size;
        }
    }
    maximun
}

pub fn print_raw_numbers(board: &Board){
   for i in 0..81 {
        let num = board.cell(i).number();
        print!("{num}");
    } 
    println!();
}


