struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
}

impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { rectangle }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for row in self.rectangle.iter_mut().skip(row1 as usize).take((row2 - row1) as usize + 1) {
            for val in row.iter_mut().skip(col1 as usize).take((col2 - col1) as usize + 1) {
                *val = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.rectangle[row as usize][col as usize] as i32
    }
}