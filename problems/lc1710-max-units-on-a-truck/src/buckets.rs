pub struct Solution;

impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, mut capacity: i32) -> i32 {
        assert!(box_types.len() <= 1000);

        let mut units = 0;
        box_types
            .into_iter()
            .fold(vec![0; 1000], |mut acc, pair| {
                acc[pair[1] as usize - 1] += pair[0];
                acc
            })
            .into_iter()
            .enumerate()
            .rev()
            .map_while(|(idx, count)| {
                let units_per_box = idx as i32 + 1;
                let boxes = std::cmp::min(capacity, count);

                capacity -= boxes;
                units += boxes * units_per_box;

                if capacity > 0 {
                    Some(())
                } else {
                    None
                }
            })
            .count();

        units
    }
}
