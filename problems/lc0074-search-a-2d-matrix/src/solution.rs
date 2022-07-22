pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if let Some(row) = pick_row(&matrix[..], target) {
            pick_cell(&row[..], target)
        } else {
            false
        }
    }
}

fn pick_row<R: AsRef<[i32]>>(rows: &[R], target: i32) -> Option<&R> {
    let mut rows = Some(rows);

    while let Some(taken) = rows.take() {
        if taken.is_empty() {
            break;
        }
        if taken.len() == 1 {
            return taken.first();
        }

        let (left, right) = taken.split_at(taken.len() / 2);
        let left_last_gte = left
            .last()
            .map(AsRef::as_ref)
            .and_then(|r| r.last())
            .copied()
            .map(|gte| gte >= target);
        let right_first_lte = right
            .first()
            .map(AsRef::as_ref)
            .and_then(|r| r.first())
            .copied()
            .map(|lte| lte <= target);

        match (left_last_gte, right_first_lte) {
            (Some(true), _) => {
                rows = Some(left);
            }
            (_, Some(true)) => {
                rows = Some(right);
            }
            _ => (),
        }
    }

    return None;
}

fn pick_cell(row: &[i32], target: i32) -> bool {
    let mut slice = Some(row);

    while let Some(taken) = slice.take() {
        if taken.is_empty() {
            break;
        }
        if taken.len() == 1 {
            return taken[0] == target;
        }

        let (left, right) = taken.split_at(taken.len() / 2);
        let left_last_gte = left.last().copied().map(|gte| gte >= target);
        let right_first_lte = right.first().copied().map(|lte| lte <= target);

        match (left_last_gte, right_first_lte) {
            (Some(true), _) => {
                slice = Some(left);
            }
            (_, Some(true)) => {
                slice = Some(right);
            }
            _ => (),
        }
    }

    false
}
