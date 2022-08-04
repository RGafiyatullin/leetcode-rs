pub struct Solution;

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        use std::cmp::Ordering::*;

        // eprintln!("IN: {:?}", arr);

        let (mut lo, mut hi) = (1, arr.len() - 2);

        let index =
        loop {
            // std::thread::sleep(std::time::Duration::from_millis(100));
            // eprintln!("[{:?}:{:?}]", lo, hi);
            
            assert!(lo <= hi);
            let mid_idx = (lo + hi) / 2;

            match (arr[mid_idx - 1].cmp(&arr[mid_idx]), arr[mid_idx].cmp(&arr[mid_idx + 1])) {
                (Less, Less) => lo = mid_idx + 1,
                (Greater, Greater) => hi = mid_idx,
                (Less, Greater) => break mid_idx,

                unexpected => panic!("Not a mountain: {:?}", unexpected),
            }
        };

        index as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: &[(&[i32], i32)] = &[
        (&[0,1,0], 1),
        (&[0,2,1,0], 1),
        (&[0,10,5,2], 1),
        (&[3,4,5,1], 2),
    ];

    #[test]
    fn run_all_cases() {
        for &(input, exp) in CASES {
            assert_eq!(Solution::peak_index_in_mountain_array(input.to_vec()), exp);
        }
    }
}