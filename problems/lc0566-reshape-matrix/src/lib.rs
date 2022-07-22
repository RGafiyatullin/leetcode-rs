pub struct Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let r = r as usize;
        let c = c as usize;

        if r * c != mat.iter().map(Vec::len).sum::<usize>() {
            mat
        } else {
            let mut items = mat.into_iter().flat_map(std::convert::identity);

            let out = (0..r)
                .map(|_| (0..c).filter_map(|_| items.next()).collect())
                .collect();

            assert!(items.next().is_none());

            out
        }
    }
}
