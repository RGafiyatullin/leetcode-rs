pub struct Solution;

const BOARD_SIZE: usize = 9;
const TRIGGERS_COUNT: usize = 3 * BOARD_SIZE;
const TRIGGER_BITS: u32 = 0b0000_0111_1111_1111_1111_1111_1111_1111;
const VALUE_BITS: u32 = 0b0000_0000_0000_0000_0000_0001_1111_1111;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut board_solver = BoardSolver::empty();
        board_solver.load(board);
        eprintln!("{}", board_solver);

        let mut iteration = 0;
        while let Some(trigger_id) = board_solver.pending_trigger() {
            iteration += 1;
            eprintln!("ITERATION: #{}", iteration);
            eprintln!(" Trigger #{} ({:?})", trigger_id, Trigger::from(trigger_id));

            let values_found = board_solver.process_trigger(trigger_id);
            eprintln!(" Values found: {:?}", values_found);
            if values_found > 0 {
                eprintln!("{}", board_solver);
            }
        }

        board_solver.save(board);
    }
}

#[derive(Debug, Clone, Copy)]
struct BoardSolver {
    cells: [[Cell; 9]; 9],
    trigger_map: [[(usize, usize); BOARD_SIZE]; TRIGGERS_COUNT],
    pending_triggers: u32,
}

impl BoardSolver {
    pub fn empty() -> Self {
        let mut trigger_map = [[(0, 0); BOARD_SIZE]; TRIGGERS_COUNT];
        let mut cells = [[Cell::new(); BOARD_SIZE]; BOARD_SIZE];

        for trigger_id in 0..TRIGGERS_COUNT {
            let trigger = Trigger::from(trigger_id);
            trigger.fill_map(&mut trigger_map[trigger_id]);

            let trigger_bit = 0b1 << trigger_id;
            for (row, col) in trigger_map[trigger_id].iter().copied() {
                cells[row][col] = cells[row][col].add_triggers(trigger_bit);
            }
        }

        Self { cells, trigger_map, pending_triggers: 0 }
    }

    pub fn load(&mut self, board: &Vec<Vec<char>>) {
        for (row_idx, row) in board.iter().enumerate() {
            for (col_idx, ch) in row.iter().copied().enumerate() {
                match ch {
                    '1'..='9' => {
                        let value = ch as u8 - '1' as u8;
                        self.set_cell_value(row_idx, col_idx, value)
                    },
                    _ => (),
                }
            }
        }
        self.pending_triggers |= TRIGGER_BITS;
    }
    pub fn save(&mut self, board: &mut Vec<Vec<char>>) {
        for (row_idx, row) in board.iter_mut().enumerate() {
            for (col_idx, ch) in row.iter_mut().enumerate() {
                *ch = if let Some(value) = self.cells[row_idx][col_idx].value() {
                    ('1' as u8 + value as u8) as char
                } else {
                    '.'
                };
            }
        }
    }

    pub fn set_cell_value(&mut self, row_idx: usize, col_idx: usize, value: u8) {
        assert!(value < BOARD_SIZE as u8);
        let value_bit = 0b1 << value;
        let cell = self.cells[row_idx][col_idx];
        self.cells[row_idx][col_idx] = cell.rm_values(VALUE_BITS).add_values(value_bit);
        self.pending_triggers |= cell.triggers();
    }

    pub fn pending_trigger(&self) -> Option<usize> {
        let mut pending_triggers = self.pending_triggers;
        for id in 0..TRIGGERS_COUNT {
            if pending_triggers & 0b1 == 0b1 {
                return Some(id)
            } else {
                pending_triggers >>= 1;
            }
        }
        None
    }
    pub fn clear_pending_trigger(&mut self, trigger_id: usize) {
        let trigger_bit: u32 = 0b1 << trigger_id;
        self.pending_triggers &= !trigger_bit;
    }
    pub fn process_trigger(&mut self, trigger_id: usize) -> usize {
        // let pending_before = self.pending_triggers;
        // eprintln!("processing trigger #{:?} ({:?})", trigger_id, Trigger::from(trigger_id));

        let values_found = self.process_trigger_once(trigger_id);

        self.clear_pending_trigger(trigger_id);
        // eprintln!("PENDING BEFORE: [{:027b}]", pending_before);
        // eprintln!("PENDING AFTER:  [{:027b}]", self.pending_triggers);

        values_found
    }

    fn process_trigger_once(&mut self, trigger_id: usize) -> usize {
        let mut values_found = 0;

        let coords = &self.trigger_map[trigger_id];

        let mut used_values = 0b0_0000_0000;

        loop {
            let used_values_before = used_values;

            for &(row, col) in coords.iter() {
                if let Some(value) = self.cells[row][col].value() {
                    let value_bit = 0b1 << value;
                    used_values |= value_bit;
                }
            }

            if used_values_before == used_values {
                break
            }

            if used_values != 0 {
                for &(row, col) in coords.iter() {
                    let cell = &mut self.cells[row][col];
                    if cell.value().is_none() {
                        let values_before = cell.values();
                        *cell = cell.rm_values(used_values);

                        if cell.values() != values_before {
                            eprintln!("[{}:{}] triggers [{:027b}]", row, col, cell.triggers());
                            self.pending_triggers |= cell.triggers();
                            values_found += 1;
                        }
                    }
                }
            }
        }

        values_found
    }
}

/**
 * Layout:
 *
 * Lower 32bits:
 *      9bits — values
 *
 * Higer 32bits
 *      27bits — triggers
 *
 */
#[derive(Debug, Clone, Copy)]
struct Cell(u64);

impl Cell {
    pub fn add_triggers(self, triggers: u32) -> Self {
        let out = Self(self.0 | ((triggers & TRIGGER_BITS) as u64) << 32);
        assert_eq!(self.values(), out.values(), "add_triggers");
        out
    }
    pub fn add_values(self, values: u32) -> Self {
        let out = Self(self.0 | (values & VALUE_BITS) as u64);
        assert_eq!(self.triggers(), out.triggers(), "add_values");
        out
    }
    pub fn rm_values(self, values: u32) -> Self {
        let out = Self(self.0 & !(values as u64));
        assert_eq!(self.triggers(), out.triggers(), "rm_values");
        out
    }
    pub fn triggers(self) -> u32 {
        (self.0 >> 32) as u32
    }

    pub fn new() -> Self {
        Self(TRIGGER_BITS as u64)
    }

    pub fn values(&self) -> u32 {
        self.0 as u32
    }

    pub fn value(&self) -> Option<usize> {
        let value_bits = self.0 as u32 & VALUE_BITS;
        if value_bits.count_ones() == 1 {
            first_set_bit(value_bits)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Trigger {
    Row(usize),
    Col(usize),
    OneNinth(usize, usize),
}

impl Trigger {
    pub fn fill_map(&self, coords: &mut [(usize, usize)]) {
        assert_eq!(coords.len(), 9);
        match self {
            &Self::Row(row) =>
                for col in 0..9 {
                    coords[col] = (row, col);
                },
            &Self::Col(col) =>
                for row in 0..9 {
                    coords[row] = (row, col);
                },
            &Self::OneNinth(row, col) => {
                let first_row = row * 3;
                let first_col = col * 3;

                let mut cells = coords.iter_mut();
                for row in first_row..(first_row + 3) {
                    for col in first_col..(first_col + 3) {
                        *cells.next().unwrap() = (row, col);
                    }
                }
            },
        }
    }
}

impl From<usize> for Trigger {
    fn from(id: usize) -> Self {
        match id {
            0..=8 => Trigger::Row(id),
            9..=17 => Trigger::Col(id - 9),
            18..=26 => {
                let qid = id - 18;
                let row_id = qid / 3;
                let col_id = qid % 3;

                Trigger::OneNinth(row_id, col_id)
            },
            _ => panic!("Invalid trigger id: {:?}", id),
        }
    }
}

impl From<Trigger> for usize {
    fn from(t: Trigger) -> Self {
        match t {
            Trigger::Row(row) => row,
            Trigger::Col(col) => col + 9,
            Trigger::OneNinth(row, col) => row * 3 + col,
        }
    }
}

fn first_set_bit(mut bits: u32) -> Option<usize> {
    for pos in 0..(std::mem::size_of_val(&bits) * 8) {
        if bits & 0b1 == 0b1 {
            return Some(pos)
        } else {
            bits >>= 1;
        }
    }
    None
}

mod impl_fmt {
    use super::*;
    use std::fmt;

    impl fmt::Display for BoardSolver {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            writeln!(f, "=== BEGIN ===")?;
            for (row_idx, row) in self.cells.iter().enumerate() {
                if row_idx % 3 == 0 && row_idx != 0 {
                    writeln!(f)?
                }

                for (col_idx, cell) in row.iter().copied().enumerate() {
                    if col_idx % 3 == 0 && col_idx != 0 {
                        write!(f, " | ")?
                    }
                    write!(f, "[{}]", cell)?;
                }

                writeln!(f)?;
            }

            writeln!(f, "PENDING: {:027b}", self.pending_triggers)?;

            writeln!(f, "===  END  ===")?;
            Ok(())
        }
    }

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let bits = 0b0001_1111_1111;
            let bits_set = self.0 & bits;

            match bits_set.count_ones() {
                0 => write!(f, "{:9}", "ERROR"),
                1 => write!(f, "={:8}", first_set_bit(bits_set as u32).unwrap() + 1),
                _ => write!(f, "{:09b}", bits_set),
            }
        }
    }
}
