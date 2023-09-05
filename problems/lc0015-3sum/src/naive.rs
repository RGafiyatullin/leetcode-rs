
pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut out = vec![];
        while let Some(third) = nums.pop() {
            two_sum(&mut out, nums.as_ref(), third);
        }
        out
    }
}

fn two_sum(out: &mut Vec<Vec<i32>>, mut nums: &[i32], third: i32) {
    while let Some((second, tail)) = nums.split_first() {
        let first = 0 - third - *second;
        
        for first in tail.iter().copied().filter(|&candidate| candidate == first) {
            out.push(vec![third, *second, first]);
        }

        nums = tail;
    }
}
