pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        is_valid_board(board).is_ok()
    }
}

fn is_valid_board(board: Vec<Vec<char>>) -> Result<(), ()> {
    let mut squares = [Nine::new(); 9];
    let mut row_states = [Nine::new(); 9];
    let mut col_states = [Nine::new(); 9];

    assert_eq!(board.len(), 9);

    for (row_idx, row) in board.into_iter().enumerate() {
        assert_eq!(row.len(), 9);

        for (col_idx, ch) in row.into_iter().enumerate() {
            if let Some(digit) = match ch {
                '0' => Some(0),
                '1' => Some(1),
                '2' => Some(2),
                '3' => Some(3),
                '4' => Some(4),
                '5' => Some(5),
                '6' => Some(6),
                '7' => Some(7),
                '8' => Some(8),
                '9' => Some(9),

                '.' => None,
                invalid => panic!("Invalid square: {}", invalid),
            } {
                let squares_idx = (row_idx / 3) * 3 + col_idx / 3;

                squares[squares_idx].add_digit(digit)?;
                row_states[row_idx].add_digit(digit)?;
                col_states[col_idx].add_digit(digit)?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Nine(u16);
impl Nine {
    pub fn new() -> Self {
        Self(0)
    }
    pub fn add_digit(&mut self, digit: u8) -> Result<(), ()> {
        let is_set = (self.0 & (1 << digit)) != 0;
        if is_set {
            return Err(())
        }

        self.0 |= 1 << digit;

        Ok(())
    }
}
