pub struct Solution;

impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let width = grid.first().map(Vec::len).unwrap_or_default();
        let mut out = (0..width as i32).collect::<Vec<_>>();

        for row in grid {
            for ball in out.iter_mut() {
                *ball = ball_step(Some(*ball), &row[..]).map(|v| v as i32).unwrap_or(-1);
            }
        }

        out
    }
}

#[inline(always)]
fn ball_step(pos_in: Option<i32>, row: &[i32]) -> Option<i32> {
    let pos = pos_in.filter(|v| *v >= 0)?;
    assert!(pos >= 0);

    let board = row.get(pos as usize).copied()?;
    let next = Some(pos + board).filter(|v| *v >= 0)?;
    let adjacent = row.get(next as usize).copied()?;
    Some(next).filter(|_| adjacent == board)
}
