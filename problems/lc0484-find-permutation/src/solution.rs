pub struct Solution;

impl Solution {
    pub fn find_permutation(input: String) -> Vec<i32> {
        assert!(input.len() >= 1);

        // eprintln!("INPUT: {:?}", input);

        let nums_count = input.len() + 1;
        let input_bytes = input.into_bytes();

        let mut corrections = vec![0; nums_count];
        for (idx, b) in input_bytes.iter().copied().enumerate().rev() {
            corrections[idx] = corrections.get(idx + 1).copied().unwrap_or_default();
            if b == 'D' as u8 {
                corrections[idx] += 1;
            }
        }

        // eprintln!("CORRECTIONS: {:?}", corrections);

        let decreases =
            std::iter::once(false).chain(input_bytes.into_iter().map(|ch| ch == 'D' as u8));
        let numbers = 1..=nums_count as i32;

        decreases
            .zip(numbers)
            .zip(corrections)
            .map(|((is_decrease, number), corr)| {
                let unadjusted = if is_decrease { 1 } else { number };
                unadjusted + corr
            })
            .collect()
    }
}
