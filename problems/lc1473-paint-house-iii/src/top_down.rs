pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_cost(houses: Vec<i32>, cost: Vec<Vec<i32>>, _m: i32, _n: i32, target: i32) -> i32 {
        assert!(houses.len() > 0);
        assert!(cost.len() == houses.len());
        assert!(target > 0);
        assert!(target <= houses.len() as i32);

        let groups_target = target as usize;

        let mut memo = Memo {
            min_cost: None,
            intermediate_costs: Default::default(),
        };

        let input = Input {
            houses: houses.as_slice(),
            cost: cost.as_slice(),
        };

        let (house, colors, tail) = input.split().expect("At least one house is expected!");
        if house != 0 {
            let next = State {
                prev: house,
                this_idx: 0,
                groups: 1,
                // solution: vec![house],
            };
            next.check(&mut memo, tail, 0, groups_target);
        } else {
            for (color_idx, color_price) in colors.iter().copied().enumerate() {
                let color = color_idx as i32 + 1;

                let next = State {
                    prev: color,
                    this_idx: 0,
                    groups: 1,
                    // solution: vec![color],
                };

                next.check(&mut memo, tail, color_price, groups_target)
            }
        }

        memo.min_cost.unwrap_or(-1)
    }
}

#[derive(Debug, Clone, Copy)]
struct Input<'a> {
    houses: &'a [i32],
    cost: &'a [Vec<i32>],
}

#[derive(Debug, Clone)]
struct State {
    prev: i32,
    this_idx: usize,
    groups: usize,
    // solution: Vec<i32>,
}

impl Input<'_> {
    fn split(&self) -> Option<(i32, &[i32], Self)> {
        let (houses_head, houses) = self.houses.split_first()?;
        let (cost_head, cost) = self.cost.split_first()?;

        Some((
            *houses_head,
            cost_head.as_ref(),
            Input {
                houses: houses,
                cost: cost,
            },
        ))
    }
}

impl State {
    fn check(&self, memo: &mut Memo, input: Input, cost_so_far: i32, groups_target: usize) {
        if self.groups > groups_target {
            // eprintln!("too many groups: {:?}", self.solution);
            return;
        }

        if let Some(min_cost) = memo.min_cost {
            if min_cost <= cost_so_far {
                return;
            }
        }

        let key = (self.prev, self.groups, self.this_idx);

        if let Some(prev_cost) = memo.intermediate_costs.get(&key) {
            if prev_cost <= &cost_so_far {
                return;
            }
        }
        memo.intermediate_costs.insert(key, cost_so_far);

        if let Some((house, colors, tail)) = input.split() {
            if house != 0 {
                let group_count = if self.prev == house {
                    self.groups
                } else {
                    self.groups + 1
                };
                let next = State {
                    prev: house,
                    this_idx: self.this_idx + 1,
                    groups: group_count,
                    // solution: {
                    //     let mut v = self.solution.clone();
                    //     v.push(house);
                    //     v
                    // },
                };
                next.check(memo, tail, cost_so_far, groups_target);
            } else {
                for (color_idx, color_price) in colors.iter().copied().enumerate() {
                    let color = color_idx as i32 + 1;

                    let group_count = if self.prev == color {
                        self.groups
                    } else {
                        self.groups + 1
                    };

                    let next = State {
                        prev: color,
                        this_idx: self.this_idx + 1,
                        groups: group_count,
                        // solution: {
                        //     let mut v = self.solution.clone();
                        //     v.push(color);
                        //     v
                        // },
                    };

                    next.check(memo, tail, cost_so_far + color_price, groups_target)
                }
            }
        } else {
            if self.groups == groups_target {
                // eprintln!("found a solution: {:?} @ {:?}", self.solution, cost_so_far);
                if let Some(min_cost) = memo.min_cost {
                    memo.min_cost = Some(std::cmp::min(cost_so_far, min_cost));
                } else {
                    memo.min_cost = Some(cost_so_far);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Memo {
    min_cost: Option<i32>,
    intermediate_costs: HashMap<(i32, usize, usize), i32>,
}
