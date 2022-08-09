pub struct Solution;

use std::collections::HashMap;

const MOD_P: usize = 1_000_000_000 + 7;

impl Solution {
    pub fn num_factored_binary_trees(mut arr: Vec<i32>) -> i32 {
        assert!(arr.len() >= 1);
        assert!(arr.len() <= 1000);

        arr.sort_unstable();

        let mut memo = HashMap::new();

        for (idx, root) in arr.iter().copied().enumerate() {
            assert!(root > 1);

            // eprintln!(" ROOT: {:?}", root);

            let count: usize = arr[0..idx]
                .iter()
                .copied()
                .map(|d| (d, root / d, root % d))
                .take_while(|&(_d, q, _r)| q > 1)
                .filter(|&(_d, _q, r)| r == 0)
                .filter_map(|(d, q, _r)| {
                    let dc = memo.get(&d).copied();
                    let qc = memo.get(&q).copied();
                    
                    // eprintln!("   {:?} = {:?} * {:?}", root, d, q);

                    let count = dc.zip(qc)
                        .map(|(dc, qc)| dc * qc)
                        .map(mod_p);

                    // eprintln!("   | {:?} -> {:?}", d, dc);
                    // eprintln!("   | {:?} -> {:?}", q, qc);
                    // eprintln!("   || count: {:?}", count);
                    
                    count
                })
                .sum::<usize>() + 1;
                
            // eprintln!("  MEMO: {:?} -> {:?}", root, count);

            memo.insert(root, count);
        }

        mod_p(memo.values().sum::<usize>()) as i32
    }
}

fn mod_p(v: usize) -> usize {
    v % MOD_P
}
