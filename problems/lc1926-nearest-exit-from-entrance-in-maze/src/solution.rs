pub struct Solution;

use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let entrance = (entrance[0] as usize, entrance[1] as usize);
        let rows = maze.len();
        let cols = maze.first().map(Vec::len).unwrap_or(0);

        maze.iter().for_each(|r| assert_eq!(cols, r.len()));

        let mut scores =
            Scores { scores: vec![usize::MAX; rows * cols], height: rows, width: cols };

        let mut queue = VecDeque::new();

        for at in (0..rows)
            .flat_map(|row| [(row, 0), (row, cols - 1)])
            .chain((0..cols).flat_map(|col| [(0, col), (rows - 1, col)]))
            .filter(|at| *at != entrance)
            .filter(|at| !is_wall(&maze, *at))
        {
            // eprintln!("start {:?}", at);
            queue.push_back((at, 0));
        }

        while let Some((at, score)) = queue.pop_front() {
            // eprintln!("=> {:?} — {:?}", at, score);
            proceed(&maze, rows, cols, &mut scores, &mut queue, at, score);
        }

        // eprintln!("ENTRANCE: {:?}", entrance);
        // eprintln!("SCORES: {:?}", scores);
        let score = scores[entrance];
        if score == usize::MAX {
            -1
        } else {
            score as i32
        }
    }
}

fn proceed(
    maze: &[Vec<char>],
    rows: usize,
    cols: usize,
    scores: &mut Scores,
    queue: &mut VecDeque<((usize, usize), usize)>,
    at: (usize, usize),
    score: usize,
) {
    if !is_wall(maze, at) && scores[at] > score {
        scores[at] = score;

        for next in neighbours(at, rows, cols) {
            queue.push_back((next, score + 1));
        }
    }
}

fn is_wall(maze: &[Vec<char>], at: (usize, usize)) -> bool {
    maze[at.0][at.1] == '+'
}

fn neighbours(
    at: (usize, usize),
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let left = at.1.checked_sub(1).map(|c| (at.0, c));
    let right = Some(at.1 + 1).filter(|c| *c < cols).map(|c| (at.0, c));
    let up = at.0.checked_sub(1).map(|r| (r, at.1));
    let down = Some(at.0 + 1).filter(|r| *r < rows).map(|r| (r, at.1));

    left.into_iter().chain(right).chain(up).chain(down)
}

#[derive(Debug)]
struct Scores {
    scores: Vec<usize>,
    height: usize,
    width: usize,
}

impl Index<(usize, usize)> for Scores {
    type Output = usize;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        assert!(index.0 < self.height);
        assert!(index.1 < self.width);

        let idx = index.0 * self.width + index.1;

        &self.scores[idx]
    }
}
impl IndexMut<(usize, usize)> for Scores {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        assert!(index.0 < self.height);
        assert!(index.1 < self.width);

        let idx = index.0 * self.width + index.1;

        &mut self.scores[idx]
    }
}
