pub struct Cell {
    index: u32,
    value: u32
}

impl Cell {
    pub fn new() -> Cell {
        let mut cell = Cell {
            index: 1,
            value: 0
        };
        cell.value = Self::calculate_value(cell.index);
        cell
    }

    pub fn get_index(&self) -> u32 {
        self.index
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    fn calculate_value(index: u32) -> u32 {
        2_i32.pow(index) as u32
    }

    pub fn update(&mut self) {
        self.index += 1;
        self.value = Self::calculate_value(self.index);
    }
}