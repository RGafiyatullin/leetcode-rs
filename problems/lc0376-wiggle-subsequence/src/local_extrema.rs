pub struct Solution;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
enum Extremum {
    Min,
    Max,
}

impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0
        }

        let pairs = nums.windows(2).map(|pair| (pair[0], pair[1]));

        let directions = pairs.filter_map(|(left, right)| match (left, right) {
            (g, l) if g > l => Some(Direction::Down),
            (l, g) if g > l => Some(Direction::Up),
            _ => None,
        });

        let extrema = directions
            .scan(None, |prev_opt, current| {
                let ext_opt = match (*prev_opt, current) {
                    (None | Some(Direction::Down), Direction::Up) => Some(Some(Extremum::Min)),
                    (None | Some(Direction::Up), Direction::Down) => Some(Some(Extremum::Max)),
                    _ => Some(None),
                };
                *prev_opt = Some(current);

                ext_opt
            })
            .filter_map(|as_is| as_is);

        extrema.count() as i32 + 1
    }
}
