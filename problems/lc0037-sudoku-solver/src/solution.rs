pub struct Solution;

type Board = [[Cell; 9]; 9];

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut solution = [[Cell::unknown(); 9]; 9];

        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, input) in row.iter().copied().enumerate() {
                match input {
                    '1'..='9' => {
                        solution[row_idx][col_idx] = Cell::known(input as u8 - '1' as u8);
                    },
                    '.' => {},
                    invalid => panic!("invalid char: {:?}", invalid),
                }
            }
        }
        dump_solution(&solution);

        for idx in 0..9 {
            update_row(&mut solution, idx);
            update_col(&mut solution, idx);
            update_rgn(&mut solution, idx);
        }

        eprintln!("===");
    }
}

fn update_row(solution: &mut Board, idx: usize) -> usize {
    update(solution, (0..9).map(|c| (idx, c)))
}

fn update_col(solution: &mut Board, idx: usize) -> usize {
    update(solution, (0..9).map(|r| (r, idx)))
}

fn update_rgn(solution: &mut Board, idx: usize) -> usize {
    // update(solution, )
    0
}

fn update(
    solution: &mut Board,
    coordinates: impl Iterator<Item = (usize, usize)> + Clone,
) -> usize {
    let mut hits = Cell::unknown();

    for (row, col) in coordinates.clone() {
        let cell = solution[row][col];
        if cell.is_known() {}
    }
    0
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Cell(u16);

impl Cell {
    pub fn unknown() -> Self {
        Self(0b0000_0001_1111_1111)
    }
    pub fn known(v: u8) -> Self {
        Self(0b0000_0000_0000_0000).add(v)
    }
    pub fn add(self, v: u8) -> Self {
        let b = 0b0000_0000_0000_0001 << v;
        Self(self.0 | b)
    }
    pub fn rm(self, v: u8) -> Self {
        let b = 0b0000_0000_0000_0001 << v;
        let b = !b;
        Self(self.0 & b)
    }
    pub fn is_set(self, v: u8) -> bool {
        self.0 & (0b0000_0000_0000_0001 << v) != 0
    }
    pub fn is_known(self) -> bool {
        self.0.count_ones() == 1
    }
    pub fn is_error(self) -> bool {
        self.0.count_ones() == 0
    }
}

use std::fmt;
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{} {:09b}]",
            if self.is_known() {
                'K'
            } else if self.is_error() {
                'E'
            } else {
                '?'
            },
            self.0
        )
    }
}

fn dump_solution(solution: &[[Cell; 9]; 9]) {
    for (row_idx, row) in solution.iter().enumerate() {
        if row_idx % 3 == 0 {
            println!()
        }
        for (col_idx, cell) in row.iter().enumerate() {
            if col_idx % 3 == 0 {
                print!(" ");
            }
            print!("{}", cell);
        }
        println!()
    }
}
