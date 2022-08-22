pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, mut stations: Vec<Vec<i32>>) -> i32 {
        stations.push(vec![target, target]);

        let result = stations
            .into_iter()
            .try_fold(State::new(start_fuel), |mut state, station| {
                assert_eq!(station.len(), 2);
                let at = station[0];
                let amount = station[1];

                // eprintln!("{:?}:{:?} [{:?}]", at, amount, state);

                assert!(at <= target);

                if at <= state.refuel_at {
                    state.stations.push(amount);
                    Ok(state)
                } else {
                    assert!(at > state.refuel_at);

                    let enough_fuel = loop {
                        if let Some(prev_amount) = state.stations.pop() {
                            state.refuel_at += prev_amount;
                            state.stations_visited += 1;

                            if state.refuel_at >= at {
                                break true;
                            }
                        } else {
                            break false;
                        }
                    };
                    if enough_fuel {
                        state.stations.push(amount);
                        Ok(state)
                    } else {
                        Err(())
                    }
                }
            });

        // eprintln!("result: {:?}", result);
        match result {
            Ok(state) if state.refuel_at >= target => state.stations_visited,
            _ => -1,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct State {
    stations: BinaryHeap<i32>,
    stations_visited: i32,
    refuel_at: i32,
}

impl State {
    pub fn new(refuel_at: i32) -> Self {
        Self {
            stations: Default::default(),
            stations_visited: 0,
            refuel_at,
        }
    }
}
