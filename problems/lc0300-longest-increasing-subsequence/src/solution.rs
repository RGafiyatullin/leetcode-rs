pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        assert!(nums.len() >= 1);
        assert!(nums.len() <= 2500);

        let mut sub = nums[0..1].to_vec();

        for i in 1..nums.len() {
            let num = nums[i];
            if num > *sub.last().expect("There is something there") {
                sub.push(num);
            } else {
                let j = binary_search(&sub, num);
                sub[j] = num;
            }
        }

        sub.len() as i32
    }
}

fn binary_search(sub: &[i32], num: i32) -> usize {
    use std::cmp::Ordering::*;

    let (mut lo, mut hi) = (0, sub.len() - 1);

    loop {
        let idx = (lo + hi) / 2;
        let val = sub[idx];

        match (lo == hi, val.cmp(&num)) {
            (false, Less) => lo = idx + 1,
            (false, Greater) => hi = idx,
            (_, Equal) => break idx,
            (true, Less | Greater) => break idx,
        }
    }
}

/*
class Solution {
    public int lengthOfLIS(int[] nums) {
        ArrayList<Integer> sub = new ArrayList<>();
        sub.add(nums[0]);

        for (int i = 1; i < nums.length; i++) {
            int num = nums[i];
            if (num > sub.get(sub.size() - 1)) {
                sub.add(num);
            } else {
                int j = binarySearch(sub, num);
                sub.set(j, num);
            }
        }

        return sub.size();
    }

    private int binarySearch(ArrayList<Integer> sub, int num) {
        int left = 0;
        int right = sub.size() - 1;
        int mid = (left + right) / 2;

        while (left < right) {
            mid = (left + right) / 2;
            if (sub.get(mid) == num) {
                return mid;
            }

            if (sub.get(mid) < num) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        return left;
    }
}
*/
