pub struct Solution;

use std::collections::{BTreeMap, BTreeSet, HashMap};

type ValuesIndex = BTreeMap<i32, BTreeSet<usize>>;

impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        // eprintln!("INPUT: {:?}", arr);

        let values_index = {
            let mut map = ValuesIndex::new();
            for (idx, value) in arr.iter().copied().enumerate() {
                map.entry(value).or_default().insert(idx);
            }
            map
        };

        // eprintln!("VAL-IDX: {:#?}", values_index);

        let last_idx = arr.len() - 1;

        let mut memo = Memo {
            good_odd: std::iter::once((last_idx, true)).collect(),
            good_even: std::iter::once((last_idx, true)).collect(),
        };

        for (idx, val) in arr.iter().copied().enumerate().rev().skip(1) {
            let after_odd_jump = make_odd_jump(&values_index, idx, val);
            let after_even_jump = make_even_jump(&values_index, idx, val);

            // eprintln!(" [{}] after-odd: {:?}; after-even: {:?}", idx, after_odd_jump,
            // after_even_jump);

            if let Some(after_odd_jump) = after_odd_jump {
                memo.good_odd.insert(
                    idx,
                    memo.good_even.get(&after_odd_jump).copied().expect(
                        format!("{:?} should have already been computed", after_odd_jump).as_str(),
                    ),
                );
            } else {
                memo.good_odd.insert(idx, false);
            }
            if let Some(after_even_jump) = after_even_jump {
                memo.good_even.insert(
                    idx,
                    memo.good_odd.get(&after_even_jump).copied().expect(
                        format!("{:?} should have already been computed", after_even_jump).as_str(),
                    ),
                );
            } else {
                memo.good_even.insert(idx, false);
            }
        }

        // eprintln!("MEMO: {:#?}", memo);
        memo.good_odd.values().filter(|b| **b).count() as i32
    }
}

#[derive(Debug, Clone, Default)]
struct Memo {
    good_even: HashMap<usize, bool>,
    good_odd: HashMap<usize, bool>,
}

fn make_odd_jump(valuvalues_index_index: &ValuesIndex, idx: usize, val: i32) -> Option<usize> {
    valuvalues_index_index
        .range(val..)
        .map(|(_val, arr_indices)| arr_indices.iter().copied())
        .flatten()
        .find(|&candidate| candidate > idx)
}

fn make_even_jump(values_index: &ValuesIndex, idx: usize, val: i32) -> Option<usize> {
    values_index
        .range(..=val)
        .rev()
        .map(|(_val, arr_indices)| arr_indices.iter().copied())
        .flatten()
        .find(|&candidate| candidate > idx)
}
