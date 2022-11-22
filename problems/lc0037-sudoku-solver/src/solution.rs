pub struct Solution;

use std::borrow::Borrow;

const BOARD_SIZE: usize = 9;
const TRIGGERS_COUNT: usize = 3 * BOARD_SIZE;
const TRIGGER_BITS: u32 = 0b0000_0111_1111_1111_1111_1111_1111_1111;
const VALUE_BITS: u32 = 0b0000_0000_0000_0000_0000_0001_1111_1111;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut board_solver = BoardSolver::empty();
        board_solver.load(board);

        // eprintln!("=== START ===");
        // eprintln!("{}", board_solver);

        let mut iterations = 0;
        board_solver.solve(&mut iterations).expect("An error at the top-level solver");
        assert!(board_solver.is_done());

        // eprintln!("=== DONE ===");
        // eprintln!("iterations: {}", iterations);
        // eprintln!("speculations: {}", board_solver.speculated.count_ones());

        // eprintln!("{}", board_solver);

        board_solver.save(board);
    }
}

#[derive(Debug, Clone, Copy)]
struct BoardSolver<TriggerMap> {
    cells: [[Cell; 9]; 9],
    trigger_map: TriggerMap,
    pending_triggers: PendingTriggers,
    speculated: u128,
    max_unsolved_cells: usize,
}
#[derive(Debug, Clone, Copy, Default)]
struct PendingTriggers {
    bits: u32,
    rotate: u32,
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

#[derive(Debug, Clone, Copy)]
enum Trigger {
    Row(usize),
    Col(usize),
    OneNinth(usize, usize),
}

#[derive(Debug)]
enum Error {
    NoPossibleValues(usize, usize, usize),
    NoProgress,
    NoSolutionFound,
    Duplicate(usize, usize, usize),
}

impl BoardSolver<[[(usize, usize); BOARD_SIZE]; TRIGGERS_COUNT]> {
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

        Self {
            cells,
            trigger_map,
            pending_triggers: Default::default(),
            speculated: 0,
            max_unsolved_cells: BOARD_SIZE * BOARD_SIZE,
        }
    }
}

impl<TM> BoardSolver<TM>
where
    TM: Borrow<[[(usize, usize); BOARD_SIZE]; TRIGGERS_COUNT]>,
{
    pub fn is_done(&self) -> bool {
        self.cells.iter().flat_map(|r| r.iter()).all(|c| c.value().is_some())
    }

    pub fn solve(&mut self, iterations_counter: &mut usize) -> Result<(), Error> {
        // let mut iteration = 0;
        while let Some(trigger_id) = self.pending_triggers.next() {
            // iteration += 1;
            *iterations_counter += 1;

            // eprintln!("ITERATION: #{} (total: #{})", iteration, *iterations_counter);
            // eprintln!(" Trigger #{} ({:?})", trigger_id, Trigger::from(trigger_id));

            let _cells_updated = self.process_trigger(trigger_id)?;
            // eprintln!(" Cells updated: {:?}", cells_updated);

            // if cells_updated > 0 {
            //     eprintln!("{}", self);
            // }
        }

        if !self.is_done() {
            // (self.cells, self.speculated) = self.speculate(iterations_counter)?;

            let _updated_cells: usize = (0..TRIGGERS_COUNT)
                .map(|trigger_id| self.process_trigger(trigger_id).unwrap())
                .sum();
            // eprintln!("Last chance updated {} cells", updated_cells);

            let (cells, speculated) = self.speculate(iterations_counter)?;
            self.cells = cells;
            self.speculated = speculated;
        }

        Ok(())
    }

    fn speculate(
        &mut self,
        iterations_counter: &mut usize,
    ) -> Result<([[Cell; BOARD_SIZE]; BOARD_SIZE], u128), Error> {
        let currently_unsolved_cells = self.unsolved_cells().count();

        // eprintln!("SPECULATE [unsolved: {}]", currently_unsolved_cells);
        // eprintln!("{}", self);

        if currently_unsolved_cells >= self.max_unsolved_cells {
            return Err(Error::NoProgress)
        }

        for (row, col, cell) in self.unsolved_cells() {
            // eprintln!(
            //     "Speculating {}:{} [cell: {}; depth: {}; iteration: #{}]",
            //     row,
            //     col,
            //     cell,
            //     self.speculated.count_ones(),
            //     *iterations_counter
            // );

            let cell_id = row * BOARD_SIZE + col;
            let speculated_bit = 0b1 << cell_id;
            let is_speculated = self.speculated & speculated_bit != 0;

            for candidate in 0..BOARD_SIZE {
                let candidate_bit = 0b1 << candidate;

                if cell.values() & candidate_bit != 0 && !is_speculated {
                    // eprintln!("  trying {:?}", candidate);

                    let mut child_solver = BoardSolver {
                        cells: self.cells,
                        trigger_map: self.trigger_map.borrow(),
                        pending_triggers: Default::default(),
                        speculated: self.speculated | speculated_bit,
                        max_unsolved_cells: currently_unsolved_cells
                            .checked_sub(1)
                            .expect("Why speculate, when there are no unsolved cells?"),
                    };
                    let speculated_cell = &mut child_solver.cells[row][col];
                    *speculated_cell =
                        speculated_cell.rm_values(VALUE_BITS).add_values(candidate_bit);
                    child_solver.pending_triggers.activate_bits(speculated_cell.triggers());

                    match child_solver.solve(iterations_counter) {
                        Ok(()) => return Ok((child_solver.cells, child_solver.speculated)),
                        Err(
                            Error::NoProgress |
                            Error::NoPossibleValues { .. } |
                            Error::NoSolutionFound |
                            Error::Duplicate { .. },
                        ) => continue,
                    }
                }
            }
        }

        Err(Error::NoSolutionFound)
    }

    fn all_cells(&self) -> impl Iterator<Item = (usize, usize, Cell)> + '_ {
        self.cells.iter().enumerate().flat_map(|(row_idx, row)| {
            row.iter().enumerate().map(move |(col_idx, cell)| (row_idx, col_idx, *cell))
        })
    }

    fn unsolved_cells(&self) -> impl Iterator<Item = (usize, usize, Cell)> + '_ {
        self.all_cells().filter(|(_, _, c)| c.value().is_none())
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
        self.pending_triggers.activate_bits(TRIGGER_BITS);
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
        self.pending_triggers.activate_bits(cell.triggers());
    }

    fn process_trigger(&mut self, trigger_id: usize) -> Result<usize, Error> {
        let mut cells_updated = 0;

        let coords = &self.trigger_map.borrow()[trigger_id];

        let mut used_values_prev = 0b0_0000_0000;

        loop {
            let mut used_values = 0b0_0000_0000;

            for &(row, col) in coords.iter() {
                if let Some(value) = self.cells[row][col].value() {
                    let value_bit = 0b1 << value;
                    if used_values & value_bit != 0 {
                        return Err(Error::Duplicate(row, col, trigger_id))
                    }
                    used_values |= value_bit;
                }
            }

            for &(row, col) in coords.iter() {
                let cell = &mut self.cells[row][col];
                if cell.value().is_none() {
                    let values_before = cell.values();
                    *cell = cell.rm_values(used_values);

                    if cell.is_error() {
                        return Err(Error::NoPossibleValues(row, col, trigger_id))
                    }

                    if cell.values() != values_before {
                        // eprintln!("[{}:{}] {:09b}=>{:09b} triggers [{:032b}]", row, col,
                        // values_before, cell.values(), cell.triggers());
                        cells_updated += 1;
                    }
                    if cell.value().is_some() {
                        self.pending_triggers.activate_bits(cell.triggers());
                    }
                }
            }

            if used_values_prev == used_values {
                break
            }
            used_values_prev = used_values;
        }

        self.pending_triggers.clear_bits(0b1 << trigger_id);
        Ok(cells_updated)
    }
}

impl PendingTriggers {
    pub fn activate_bits(&mut self, triggers_bits: u32) {
        let trigger_bits = triggers_bits.rotate_right(self.rotate);
        self.bits |= trigger_bits;
    }
    pub fn clear_bits(&mut self, trigger_bits: u32) {
        let trigger_bits = trigger_bits.rotate_right(self.rotate);
        self.bits &= !trigger_bits;
    }

    pub fn trigger_bits(&self) -> u32 {
        self.bits.rotate_left(self.rotate)
    }

    pub fn next(&mut self) -> Option<usize> {
        for _ in 0..32 {
            let bits = self.bits;
            let rotate = self.rotate;
            self.shift_right();
            if bits & 0b1 == 0b1 {
                return Some(rotate as usize % 32)
            }
        }
        None
    }

    fn shift_right(&mut self) {
        self.bits >>= 1;
        self.rotate += 1;
        // self.rotate %= 32;
    }
}

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

    pub fn is_error(&self) -> bool {
        self.values().count_ones() == 0
    }

    pub fn values(&self) -> u32 {
        self.0 as u32
    }

    pub fn value(&self) -> Option<usize> {
        let mut value_bits = self.0 as u32 & VALUE_BITS;
        if value_bits.count_ones() == 1 {
            for pos in 0..(std::mem::size_of_val(&value_bits) * 8) {
                if value_bits & 0b1 == 0b1 {
                    return Some(pos)
                } else {
                    value_bits >>= 1;
                }
            }
            None
        } else {
            None
        }
    }
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

mod impl_fmt {
    use super::*;
    use std::fmt;

    impl<TM> fmt::Display for BoardSolver<TM> {
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

            writeln!(f, "PENDING: {}", self.pending_triggers)?;
            writeln!(f, "SPECULATED: {:081b}", self.speculated)?;

            writeln!(f, "===  END  ===")?;
            Ok(())
        }
    }

    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_error() {
                write!(f, "{:9}", "ERROR")
            } else if let Some(value) = self.value() {
                write!(f, "={:8}", value + 1)
            } else {
                let bits = 0b0001_1111_1111;
                let bits_set = self.0 & bits;
                write!(f, "{:09b}", bits_set)
            }
        }
    }

    impl fmt::Display for PendingTriggers {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "[{:028b}; bits: {:032b}; rotate: {:?}]",
                self.trigger_bits(),
                self.bits,
                self.rotate
            )
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::NoPossibleValues(row, col, trigger) => write!(
                    f,
                    "No possible values [at: {}:{}; trigger: {:?}]",
                    row,
                    col,
                    Trigger::from(*trigger)
                ),
                Self::Duplicate(row, col, trigger) => write!(
                    f,
                    "Duplicate [at: {}:{}; trigger: {:?}]",
                    row,
                    col,
                    Trigger::from(*trigger)
                ),
                Self::NoProgress => write!(f, "No progress done"),
                Self::NoSolutionFound => write!(f, "No solution found"),
            }
        }
    }
}
