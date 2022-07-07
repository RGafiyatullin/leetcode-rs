pub struct Solution;

use std::collections::{btree_map::Entry, BTreeMap, BTreeSet, HashSet};

impl Solution {
    pub fn num_islands(land_map: Vec<Vec<char>>) -> i32 {
        let mut next_isle_id: usize = 0;
        let mut isles: Vec<Vec<Option<usize>>> = Vec::with_capacity(land_map.len());
        let mut isle_connections: BTreeMap<usize, BTreeSet<usize>> = Default::default();

        for land_row in land_map.iter() {
            let mut isles_row = Vec::with_capacity(land_row.len());
            for (col_idx, is_land) in land_row.iter().map(|c| *c == '1').enumerate() {
                match (
                    is_land,
                    isles_row.last().map(Option::as_ref).flatten().copied(),
                    isles.last().map(|r| r[col_idx]).flatten(),
                ) {
                    (false, _, _) => isles_row.push(None),

                    (true, Some(isle_id), None) => {
                        // eprintln!("L: {}", isle_id);
                        isles_row.push(Some(isle_id))
                    }
                    (true, None, Some(isle_id)) => {
                        // eprintln!("U: {}", isle_id);
                        isles_row.push(Some(isle_id))
                    }
                    (true, Some(left_isle_id), Some(upper_isle_id))
                        if left_isle_id == upper_isle_id =>
                    {
                        // eprintln!("L&U: {}", left_isle_id);
                        isles_row.push(Some(left_isle_id))
                    }

                    (true, Some(left_isle_id), Some(upper_isle_id)) => {
                        assert_ne!(left_isle_id, upper_isle_id);

                        // eprintln!("L: {}; U: {}", left_isle_id, upper_isle_id);

                        let lo_isle_id = std::cmp::min(left_isle_id, upper_isle_id);
                        let hi_isle_id = std::cmp::max(left_isle_id, upper_isle_id);

                        match isle_connections.entry(lo_isle_id) {
                            Entry::Vacant(entry) => {
                                entry.insert(std::iter::once(hi_isle_id).collect());
                            }
                            Entry::Occupied(mut entry) => {
                                entry.get_mut().insert(hi_isle_id);
                            }
                        }

                        isles_row.push(Some(lo_isle_id));
                    }

                    (true, None, None) => {
                        let new_isle_id = next_isle_id;
                        next_isle_id += 1;

                        // eprintln!("NEW ISLAND: {:?}", new_isle_id);

                        isles_row.push(Some(new_isle_id));
                    }
                }
            }
            isles.push(isles_row);
        }

        let mut isles = (0..next_isle_id).collect::<Vec<_>>();

        for (&lo_id, hi_ids) in isle_connections.range(..) {
            eprintln!("lo_id: {} => {:?}", lo_id, hi_ids);
            let target_id = if let Some(min_id) = hi_ids.iter().map(|&id| isles[id]).min() {
                eprintln!("  // target_id = min( {}, {} )", min_id, lo_id);
                std::cmp::min(min_id, isles[lo_id])
            } else {
                eprintln!("  // target_id = {}", isles[lo_id]);
                isles[lo_id]
            };
            eprintln!("  target_id = {}", target_id);
            isles[lo_id] = target_id;
            for &hi_id in hi_ids.range(..) {
                assert!(hi_id > lo_id);
                isles[hi_id] = target_id;
            }
        }

        eprintln!("===");
        for (from, to) in isles.iter().copied().enumerate() {
            eprintln!("{} => {}", from, to);
        }

        isles.into_iter().collect::<HashSet<_>>().len() as i32
    }
}
