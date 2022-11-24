pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let row_count = board.len();
        let col_count = board.first().map(Vec::len).unwrap_or(0);
        assert!(board.iter().all(|r| r.len() == col_count));

        let mut visit_map = vec![false; col_count * row_count];

        (0..row_count).flat_map(|r| (0..col_count).map(move |c| (r, c))).any(|(r, c)| {
            word_search(&mut visit_map, (r, c), (row_count, col_count), &board, word.chars())
        })
    }
}

fn word_search(
    visit_map: &mut [bool],
    (row_idx, col_idx): (usize, usize),
    (row_count, col_count): (usize, usize),
    board: &[Vec<char>],
    mut word: impl Iterator<Item = char> + Clone,
) -> bool {
    let visit_idx = row_idx * col_count + col_idx;
    if visit_map[visit_idx] {
        false
    } else if let Some(expected_char) = word.next() {
        visit_map[visit_idx] = true;
        let out = board[row_idx][col_idx] == expected_char &&
            (word.clone().next().is_none() ||
                row_idx
                    .checked_sub(1)
                    .map(|r| (r, col_idx))
                    .into_iter()
                    .chain(col_idx.checked_sub(1).map(|c| (row_idx, c)))
                    .chain(Some(row_idx + 1).filter(|r| *r < row_count).map(|r| (r, col_idx)))
                    .chain(Some(col_idx + 1).filter(|c| *c < col_count).map(|c| (row_idx, c)))
                    .any(|(r, c)| {
                        word_search(visit_map, (r, c), (row_count, col_count), board, word.clone())
                    }));
        visit_map[visit_idx] = false;
        out
    } else {
        true
    }
}
