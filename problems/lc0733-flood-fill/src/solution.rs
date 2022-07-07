pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let pos = (sr as usize, sc as usize);
        let mut visit_plan = VisitPlan::default();
        visit_plan.extend(std::iter::once(pos));

        while let Some(pos) = visit_plan.next() {
            visit(&mut visit_plan, &mut image, pos, color);
        }

        image
    }
}

fn visit<V>(visit_plan: &mut V, image: &mut Vec<Vec<i32>>, pos: (usize, usize), target_color: i32)
where
    V: Extend<(usize, usize)>,
{
    let (row, col) = pos;
    let orig_color = image[row][col];
    image[row][col] = target_color;

    if row != 0 && image[row - 1][col] == orig_color {
        visit_plan.extend(std::iter::once((row - 1, col)));
    }
    if col != 0 && image[row][col - 1] == orig_color {
        visit_plan.extend(std::iter::once((row, col - 1)));
    }
    if row < image.len() - 1 && image[row + 1][col] == orig_color {
        visit_plan.extend(std::iter::once((row + 1, col)));
    }
    if col < image[row].len() - 1 && image[row][col + 1] == orig_color {
        visit_plan.extend(std::iter::once((row, col + 1)));
    }
}

#[derive(Debug, Clone, Default)]
struct VisitPlan {
    visited: HashSet<(usize, usize)>,
    to_visit: Vec<(usize, usize)>,
}

impl Iterator for VisitPlan {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        self.to_visit.pop()
    }
}

impl Extend<(usize, usize)> for VisitPlan {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (usize, usize)>,
    {
        for item in iter {
            if !self.visited.contains(&item) {
                self.visited.insert(item);
                self.to_visit.push(item);
            }
        }
    }
}
