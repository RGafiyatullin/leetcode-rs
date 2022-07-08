pub struct Solution;

/// THIS IF FUCKING HIDEOUS!!!

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, m: i32, n: i32, target: i32) -> i32 {
        assert!(houses.len() > 0);
        assert!(cost.len() == houses.len());
        assert!(target > 0);
        assert!(target <= houses.len() as i32);

        let m = m as usize;
        let n = n as usize;
        let target = target as usize;

        let mut prev_memo: Vec<Vec<Option<i32>>> = vec![vec![None; n]; target + 1];

        for color in 1..=n {
            if houses[0] as usize == color {
                prev_memo[1][color - 1] = Some(0);
            } else if houses[0] == 0 {
                prev_memo[1][color - 1] = Some(cost[0][color - 1]);
            }
        }

        for house in 1..m {
            let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; n]; target + 1];

            for neighbourhoods in 1..=std::cmp::min(target, house + 1) {
                for color in 1..=n {
                    if houses[house] != 0 && color != houses[house] as usize {
                        continue;
                    }

                    let curr_cost = (1..=n)
                        .map(|prev_color| {
                            if prev_color != color {
                                prev_memo[neighbourhoods - 1][prev_color - 1]
                            } else {
                                prev_memo[neighbourhoods][color - 1]
                            }
                        })
                        .filter_map(std::convert::identity)
                        .min();

                    memo[neighbourhoods][color - 1] = curr_cost.map(|curr_cost| {
                        curr_cost
                            + if houses[house] != 0 {
                                0
                            } else {
                                cost[house][color - 1]
                            }
                    });
                }
            }
            prev_memo = memo;
        }

        prev_memo[target]
            .iter()
            .copied()
            .filter_map(std::convert::identity)
            .min()
            .unwrap_or(-1) as i32
    }
}
