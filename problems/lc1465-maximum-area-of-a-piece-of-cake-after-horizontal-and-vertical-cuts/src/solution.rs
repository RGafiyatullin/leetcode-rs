pub struct Solution;

impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        horizontal_cuts.sort();
        vertical_cuts.sort();

        let max_height = get_max_interval(h, horizontal_cuts.as_ref())
            .expect("At least one piece is guaranteed to be: [0; h]")
            as u64;
        let max_width = get_max_interval(w, vertical_cuts.as_ref())
            .expect("At least one piece is guaranteed to be: [0; w]")
            as u64;

        let max_area = max_height * max_width;

        (max_area % 1000000007) as i32
    }
}

fn get_max_interval(l: i32, cuts: &[i32]) -> Option<i32> {
    // eprintln!("l: {}; cuts: {:?}", l, cuts);

    std::iter::once(0)
        .chain(cuts.iter().cloned())
        .chain(std::iter::once(l))
        // .inspect(|c| eprintln!("cut: {:?}", c))
        .scan(Option::<i32>::None, |prev_opt, this| match *prev_opt {
            None => {
                *prev_opt = Some(this);
                Some(None)
            },
            Some(prev) => {
                *prev_opt = Some(this);
                Some(Some(this - prev))
            }
        })
        // .inspect(|i| eprintln!("int: {:?}", i))
        .filter_map(std::convert::identity)
        .max()
}
