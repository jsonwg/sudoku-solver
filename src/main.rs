fn main() {
    let mut board = Board {
        board: [[0u8; 9]; 9],
        row: 0,
        col: 0,
    };
    board.solve();
}

struct Board {
    board: [[u8; 9]; 9],
    row: usize,
    col: usize,
}

impl Board {
    fn value(&self) -> u8 {
        self.board[self.row][self.col]
    }

    fn validate_row(&self) -> bool {
        self.board[self.row]
            .iter()
            .filter(|&n| *n == self.value())
            .count()
            > 1
    }

    fn validate_col(&self) -> bool {
        self.board
            .iter()
            .map(|r| r[self.col])
            .filter(|&n| n == self.value())
            .count()
            > 1
    }

    fn validate_box(&self) -> bool {
        let box_row = match self.row {
            0..=2 => 0,
            3..=5 => 3,
            6..=8 => 6,
            _ => panic!("Row number is out of bounds"),
        };

        let box_col = match self.col {
            0..=2 => 0,
            3..=5 => 3,
            6..=8 => 6,
            _ => panic!("Column number is out of bounds"),
        };

        self.board[box_row..box_row + 2]
            .iter()
            .map(|r| &r[box_col..box_col + 2])
            .flatten()
            .filter(|&n| *n == self.value())
            .count()
            > 1
    }

    fn solve(&mut self) {
        if self.validate_row() || self.validate_col() || self.validate_box() {
            return;
        }
        self.solve()
    }
}
