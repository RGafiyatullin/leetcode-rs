pub struct Solution;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let mut maze = Maze::try_from(maze).expect("Bad maze");
        maze.set_entrance(entrance);
        eprintln!("{}", maze);

        

        unimplemented!()
    }
}

#[derive(Debug)]
struct Maze {
    cells: Vec<Cell>,
    cols: usize,
    rows: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Exit,
    Entrance,
    Path(usize),
}

impl Maze {
    pub fn set_entrance(&mut self, entrance: (usize, usize)) {
        *self.get_mut(entrance) = Cell::Entrance;
    }
    fn idx(&self, coordinates: (usize, usize)) -> usize {
        assert!(coordinates.1 < self.cols);
        assert!(coordinates.0 < self.rows);

        coordinates.0 * self.cols + coordinates.1
    }
    pub fn get(&self, coordinates: (usize, usize)) -> Cell {
        let idx = self.idx(coordinates);
        self.cells[idx]
    }
    pub fn get_mut(&mut self, coordinates: (usize, usize)) -> &mut Cell {
        let idx = self.idx(coordinates);
        &mut self.cells[idx]
    }
}

impl std::fmt::Display for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row_idx in 0..self.rows {
            for col_idx in 0..self.cols {
                write!(f, "{}", self.get((row_idx, col_idx)))?;
            }
            writeln!(f)?
        }
        Ok(())
    }
}
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Entrance => write!(f, "[ I ]"),
            Self::Exit => write!(f, "[ O ]"),
            Self::Path(v) if *v == usize::MAX => write!(f, "[ ? ]"),
            Self::Path(v) => write!(f, "[{:3}]", v),
            Self::Wall => write!(f, "[ W ]"),
        }
    }
}

impl TryFrom<Vec<Vec<char>>> for Maze {
    type Error = &'static str;
    fn try_from(rows: Vec<Vec<char>>) -> Result<Self, Self::Error> {
        let height = rows.len();
        let width = if let Some(width) = rows.first().map(Vec::len) {
            for row in rows.iter() {
                if row.len() != width {
                    return Err("the maze is not rectangular")
                }
            }
            width
        } else {
            0
        };

        let mut cells = Vec::with_capacity(height * width);

        for (row_idx, row) in rows.into_iter().enumerate() {
            for (col_idx, ch) in row.into_iter().enumerate() {
                let is_edge =
                    row_idx == 0 || row_idx == height - 1 || col_idx == 0 || col_idx == width - 1;
                cells.push(match (ch, is_edge) {
                    ('+', _) => Cell::Wall,
                    ('.', true) => Cell::Exit,
                    ('.', false) => Cell::Path(usize::MAX),
                    (_, _) => return Err("invalid char"),
                });
            }
        }

        let out = Self { cells, cols: width, rows: height };

        Ok(out)
    }
}
