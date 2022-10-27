pub struct Solution;

impl Solution {
    pub fn array_strings_are_equal(left: Vec<String>, right: Vec<String>) -> bool {
        let mut left = left.iter().flat_map(|s| s.chars());
        let mut right = right.iter().flat_map(|s| s.chars());

        while let Some(l) = left.next() {
            if let Some(r) = right.next() {
                if l != r {
                    return false
                }
            } else {
                return false
            }
        }

        right.next().is_none()
    }
}
