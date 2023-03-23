pub struct Solution;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        assert!(n >= 1 && n <= 100_000);
        let n = n as usize;

        let mut nodes = (0..n).collect::<Vec<_>>();
        let mut cables = vec![0usize; n];

        for conn in connections {
            if let &[left, right] = &conn[..] {
                let lo = left.min(right) as usize;
                let hi = left.max(right) as usize;

                assert_ne!(lo, hi);
                assert!(lo < n);
                assert!(hi < n);

                let set_id_lo = nodes[lo];
                let set_id_hi = nodes[hi];
                let new_set_id = set_id_lo.min(set_id_hi);

                nodes.iter_mut().for_each(|set_id| {
                    if *set_id == set_id_lo || *set_id == set_id_hi {
                        *set_id = new_set_id;
                    }
                });

                cables[new_set_id] += 1;
            } else {
                panic!("bad connection: {:?}", conn);
            }
        }

        let mut set_sizes = vec![0usize; n];

        for node_id in 0..n {
            if nodes[node_id] != node_id {
                let set_id = nodes[nodes[node_id]];

                nodes[node_id] = set_id;
                cables[set_id] += std::mem::take(&mut cables[node_id]);

                set_sizes[set_id] += 1;
            } else {
                set_sizes[node_id] = 1;
            }
        }

        // eprintln!("NODES:  {:?}", nodes);
        // eprintln!("SIZES:  {:?}", set_sizes);
        // eprintln!("CABLES: {:?}", cables);

        let mut isolated_sets = 0usize;
        let mut spare_cables = 0usize;

        for set_id in 0..n {
            let set_size = set_sizes[set_id];

            if set_size > 0 {
                // eprintln!(" + set#{:?}", set_id);
                isolated_sets += 1;

                let cables_in_set = cables[set_id];
                // eprintln!("   cables: {:?}", cables_in_set);

                let needed_cables = set_size - 1;
                // eprintln!("   needed: {:?}", needed_cables);

                let spare = cables_in_set - needed_cables;
                // eprintln!("   spare: {:?}", spare);

                spare_cables += spare;
            }
        }

        // eprintln!("SPARE: {:?}; ISLES: {:?}", spare_cables, isolated_sets);

        let changes_needed = isolated_sets - 1;

        if changes_needed <= spare_cables {
            changes_needed as i32
        } else {
            -1
        }
    }
}
