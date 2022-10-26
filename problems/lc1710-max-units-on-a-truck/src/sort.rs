pub struct Solution;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut capacity: i32) -> i32 {
        box_types.sort_by_key(|pair| pair[1]);

        let mut units = 0;

        while let Some(pair) = box_types.pop() {
            if capacity <= 0 {
                break
            }

            let c = pair[0];
            let t = pair[1];

            let boxes = std::cmp::min(c, capacity);

            capacity -= boxes;
            units += boxes * t;
        }

        units
    }
}
