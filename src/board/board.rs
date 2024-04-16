use crate::cell::cell::Cell;
use rand::Rng;

pub struct Board {
    pub cells: Vec<Vec<Option<Cell>>>
}

enum Option<Cell> {
    Some(Cell),
    None
}

impl Board {
    pub fn new() -> Board {
        let std_index = 4;
        let mut board = Self::from(std_index);
        for _ in 0..2 {
            let mut rng = rand::thread_rng();
            let rand_x = rng.gen_range(0..std_index-1);
            let rand_y = rng.gen_range(0..std_index-1);
            let opt_cell:Option<Cell> =  Option::Some(Cell::new());
            board.cells[rand_x as usize][rand_y as usize] = opt_cell
        }
        board
    }

    pub fn print(&self) {
        let mut cell_width = 0;
        for row in &self.cells {
            for opt_cell in row {
                match opt_cell {
                    Option::Some(cell) => {
                        if cell.get_value() > cell_width {
                            cell_width = cell.get_value();
                        }
                    },
                    Option::None => {
                        continue
                    }
                }
            }
        }
        let cell_width_padded = cell_width + 2;
        for row in &self.cells {
            Self::print_row_divider((self.cells.len() as u32) * cell_width_padded);
            Self::print_row(row, cell_width_padded);
        }
        Self::print_row_divider((self.cells.len() as u32) * cell_width_padded);
    }

    fn print_row(row: &Vec<Option<Cell>>, cellsize: u32) {
        let cellsize = cellsize - 1;
        for _ in 0..cellsize/2 {
            Self::print_empty_non_number_line(row.len() as u32, cellsize);

        }
        for cell in row {
            print!("|");
            match cell {
                Option::Some(cell) => {
                    Self::print_number_cell(cell.get_value(), cellsize);
                },
                Option::None => {
                    Self::print_number_cell(0, cellsize);
                }
            }
        }
        println!("|");
        for _ in 0..cellsize/2 {
            Self::print_empty_non_number_line(row.len() as u32, cellsize);
        }
    }

    fn print_row_divider(length: u32) {
        for _ in 0..length {
            print!("-")
        }
        println!()
    }

    fn print_number_cell(value: u32, cellsize: u32) {
        for _ in 0..(cellsize-value.to_string().len() as u32)/2 {
            print!(" ")
        }
        print!("{}", value);
        for _ in 0..(cellsize-value.to_string().len() as u32)/2 {
            print!(" ")
        }
    }
    fn print_empty_non_number_line(amount_of_cells: u32, cellsize: u32) {
        for _ in 0..amount_of_cells {
            print!("|");
            for _ in 0..cellsize {
                print!(" ");
            }
        }
        println!("|")
    }

    pub fn new_specific(index: u32) -> Board {
        let mut board = Self::from(index);
        for _ in 0..2 {
            let mut rng = rand::thread_rng();
            let rand_x = rng.gen_range(0..index);
            let rand_y = rng.gen_range(0..index);
            board.cells[rand_x as usize][rand_y as usize] = Option::Some(Cell::new());
        }
        board
    }

    fn from(index: u32) -> Board {
        let mut cells = Vec::new();
        for _ in 0..index {
            let mut row = Vec::new();
            for _ in 0..index {
                let optional_cell: Option<Cell> = Option::None;
                row.push(optional_cell);
            }
            cells.push(row);
        }
        Board {
            cells: cells
        }
    }
}