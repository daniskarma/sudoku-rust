#![allow(dead_code)]
use crate::auxiliar;
use core::panic;

#[derive(Clone, PartialEq)]
pub struct Cell {
    number: u8,
    original: bool,
    id: usize,
    options: Vec<u8>,
}

impl Cell {
    fn new(number: u8, original: bool, id: usize) -> Self {
        let cell_options: Vec<u8> = if original {
            vec![number]
        } else {
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
        };

        Cell {
            number,
            original,
            id,
            options: cell_options,
        }
    }
    pub fn number(&self) -> u8 {
        self.number
    }
    pub fn original(&self) -> bool {
        self.original
    }
    pub fn options(&self) -> &Vec<u8> {
        &self.options
    }
    pub fn id(&self) -> usize {
        self.id
    }
    /// returns the row number of the cell
    pub fn row(&self) -> usize {
        cell_to_row(self.id)
    }
    /// returns the col number of the cell
    pub fn col(&self) -> usize {
        cell_to_col(self.id)
    }
    /// returns the sqr number of the cell
    pub fn sqr(&self) -> usize {
        cell_to_sqr(self.id)
    }

    pub fn set_number(&mut self, number: u8) {
        if number > 9 {
            panic!("Number must lower than 9")
        }
        self.number = number;
    }
    pub fn set_options(&mut self, options: Vec<u8>) {
        if options.len() > 8 {
            panic!("Options cannot have more than 8 elements.")
        }
        if !auxiliar::all_unique(&options) {
            panic!("Options elements must be unique.")
        }
        self.options = options;
    }
}

/// Represents a sudoku board of 9x9
#[derive(Clone, PartialEq)]
pub struct Board {
    /// Vector of 81 Cells
    cells: Vec<Cell>,
}
#[allow(dead_code)]
impl Board {
    /// Create a board given a sudoku codified as a 81 characters string.
    /// # Arguments
    /// * `sudoku` - A string slice that holds the sudoku we want to create. Must have 81
    /// characters.
    pub fn create(sudoku: &str) -> Self {
        let mut board_cells: Vec<Cell> = Vec::new();
        if sudoku.len() != 81 {
            panic!("sudoku must have 81 characters")
        }
        for (i, char) in sudoku.char_indices() {
            if !"0123456789".contains(char) {
                panic!("sudoku must be only numbers")
            }
            let is_original: bool = char != '0';
            board_cells.push(Cell::new(char.to_digit(10).unwrap() as u8, is_original, i));
        }
        Board { cells: board_cells }
    }
    /// Getter for  a Cell. Returns a mutable reference, so Cell attributes can be modified.
    pub fn mut_cell(&mut self, n: usize) -> &mut Cell {
        if n > 80 {
            panic!("Cell number mut be less than 81.")
        }
        &mut self.cells[n]
    }
    pub fn cell(&self, n: usize) -> &Cell {
        if n > 80 {
            panic!("Cell number mut be less than 81.")
        }
        &self.cells[n]
    }

    /// Getter for a vec of all cells.
    pub fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    /// Getter for a row. Returns an inmutable reference, so Cells cannot be modified.
    /// Rows are listed up to down, 0 to 8.
    pub fn row(&self, n: usize) -> Vec<&Cell> {
        if n > 8 {
            panic!("row number must be less than 9")
        }
        let mut row = Vec::new();
        let row_start = n * 9;
        for i in 0..9 {
            row.push(&self.cells[row_start + i])
        }
        row
    }
    /// Getter for a row. Returns an inmutable reference, so Cells cannot be modified.
    /// Get the row from a cell position, 0 to 80.
    pub fn row_fr_cell(&self, n: usize) -> Vec<&Cell> {
        let nr = cell_to_row(n);
        self.row(nr)
    }

    /// Getter for a column. Returns an inmutable reference, so Cells cannot be modified.
    /// Collumns are listed left to right, 0 to 8.
    pub fn col(&self, n: usize) -> Vec<&Cell> {
        if n > 8 {
            panic!("col number must be less than 9")
        }
        let mut col = Vec::new();
        let col_start = n;
        for i in 0..9 {
            col.push(&self.cells[col_start + 9 * i])
        }
        col
    }
    /// Getter for a column. Returns an inmutable reference, so Cells cannot be modified.
    /// Get the column from a cell position, 0 to 80.
    pub fn col_fr_cell(&self, n: usize) -> Vec<&Cell> {
        let nc = cell_to_col(n);
        self.col(nc)
    }

    /// Getter for a Square. Returns an inmutable reference, so Cells cannot be modified.
    /// Squares are listed uper left to botton right. Same for Cells inside a Box.
    pub fn sqr(&self, n: usize) -> Vec<&Cell> {
        if n > 8 {
            panic!("box number must be less than 9")
        }
        let mut sqr = Vec::new();
        let nx = n % 3;
        let ny = n / 3;
        let sqr_start = nx * 3 + ny * 27;
        for i in 0..3 {
            for j in 0..3 {
                sqr.push(&self.cells[sqr_start + j + i * 9])
            }
        }
        sqr
    }
    /// Getter for a Square. Returns an inmutable reference, so Cells cannot be modified.
    /// Squares are listed uper left to botton right. Same for Cells inside a Box.
    /// Get the square from a cell position, 0 to 80.
    pub fn sqr_fr_cell(&self, n: usize) -> Vec<&Cell> {
        let ns = cell_to_sqr(n);
        self.sqr(ns)
    }

    /// Getter for the numbers of a row.
    /// Takes row number 0 to 8, up to down.
    pub fn row_num(&self, n: usize) -> Vec<u8> {
        self.row(n).iter().map(|x| x.number()).collect()
    }
    /// Getter for the numbers of a row.
    /// Takes cell number, 0 to 80, and returns its row.
    pub fn row_num_fr_cell(&self, n: usize) -> Vec<u8> {
        let nr = cell_to_row(n);
        self.row_num(nr)
    }

    /// Getter for the numbers of a column.
    /// Takes column number 0 to 8, left to right.
    pub fn col_num(&self, n: usize) -> Vec<u8> {
        self.col(n).iter().map(|x| x.number()).collect()
    }
    /// Getter for the numbers of a column.
    /// Takes cell number, 0 to 80, and returns its column.
    pub fn col_num_fr_cell(&self, n: usize) -> Vec<u8> {
        let nc = cell_to_col(n);
        self.col_num(nc)
    }

    /// Getter for the numbers of a square.
    /// Takes square number 0 to 8, top left to botton right.
    pub fn sqr_num(&self, n: usize) -> Vec<u8> {
        self.sqr(n).iter().map(|x| x.number()).collect()
    }
    /// Getter for the numbers of a square.
    /// Takes cell number, 0 to 80, and returns its square.
    pub fn sqr_num_fr_cell(&self, n: usize) -> Vec<u8> {
        let ns = cell_to_sqr(n);
        self.sqr_num(ns)
    }

    /// Getter for the options of a row.
    /// Takes row number 0 to 8, up to down.
    pub fn row_opt(&self, n: usize) -> Vec<Vec<u8>> {
        self.row(n).iter().map(|x| x.options().to_owned()).collect()
    }
    /// Getter for the options of a row.
    /// Takes cell number, 0 to 80, and returns its row.
    pub fn row_opt_fr_cell(&self, n: usize) -> Vec<Vec<u8>> {
        let nr = cell_to_row(n);
        self.row_opt(nr)
    }
    /// Getter for the options of a column.
    /// Takes column number 0 to 8, left to right.
    pub fn col_opt(&self, n: usize) -> Vec<Vec<u8>> {
        self.col(n).iter().map(|x| x.options().to_owned()).collect()
    }
    /// Getter for the options of a column.
    /// Takes cell number, 0 to 80, and returns its column.
    pub fn col_opt_fr_cell(&self, n: usize) -> Vec<Vec<u8>> {
        let nc = cell_to_col(n);
        self.col_opt(nc)
    }
    /// Getter for the options of a square.
    /// Takes square number 0 to 8, top left to botton right.
    pub fn sqr_opt(&self, n: usize) -> Vec<Vec<u8>> {
        self.sqr(n).iter().map(|x| x.options().to_owned()).collect()
    }
    /// Getter for the options of a square.
    /// Takes cell number, 0 to 80, and returns its square.
    pub fn sqr_opt_fr_cell(&self, n: usize) -> Vec<Vec<u8>> {
        let ns = cell_to_sqr(n);
        self.sqr_opt(ns)
    }

    pub fn cells_to_opts(&self, region_cells: &Vec<&Cell>) -> Vec<Vec<u8>> {
        let mut region_opts: Vec<Vec<u8>> = vec![];
        for cell in region_cells {
            let opts = cell.options();
            region_opts.push(opts.to_vec());
        }
        region_opts
    }
}

fn cell_to_row(n: usize) -> usize {
    n / 9
}
fn cell_to_col(n: usize) -> usize {
    n % 9
}
fn cell_to_sqr(n: usize) -> usize {
    let nr = n / 9;
    let nc = n % 9;
    (nc / 3) + (nr / 3) * 3
}
