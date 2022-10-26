pub struct Solution;

use std::cmp::Ordering;

#[derive(Debug)]
struct Scanner {
    last_in: i32,
    last_out: usize,

    last_drop: Option<usize>,
    timer: Option<usize>,

    acc: usize,
    // debug: Vec<usize>,
}

impl Scanner {
    pub fn init(first: i32) -> Self {
        Self {
            last_in: first,
            last_out: 1,

            last_drop: None,
            timer: None,

            acc: 1,
            // debug: vec![1],
        }
    }

    pub fn scan(&mut self, current_idx: usize, current_in: i32) {
        match current_in.cmp(&self.last_in) {
            Ordering::Greater => {
                // eprintln!("Gt");
                // eprintln!(" in:  {:?}", self);
                let current_output = self.last_out + 1;

                self.last_drop = None;
                self.timer = None;

                self.last_in = current_in;
                self.last_out = current_output;
                self.acc += current_output;

                // self.debug.push(current_output);

                // eprintln!(" out: {:?}", self);
            },
            Ordering::Equal => {
                // eprintln!("Eq");
                // eprintln!(" in:  {:?}", self);
                self.last_drop = Some(current_idx);
                self.timer = None;

                self.last_in = current_in;
                self.last_out = 1;

                self.acc += 1;

                // self.debug.push(1);

                // eprintln!(" out: {:?}", self);
            },
            Ordering::Less => {
                // eprintln!("Lt None");
                // eprintln!(" in:  {:?}", self);

                let last_drop = if let Some(last_drop) = self.last_drop {
                    last_drop
                } else {
                    self.timer = Some(self.last_out - 1);
                    current_idx
                };

                let last_drop = match self.timer {
                    None => last_drop,
                    Some(0) => {
                        self.timer = None;
                        last_drop - 1
                    },
                    Some(n) => {
                        self.timer = Some(n - 1);
                        last_drop
                    },
                };

                self.last_drop = Some(last_drop);

                self.last_in = current_in;
                self.last_out = 1;

                self.acc += current_idx - last_drop + 1;

                // for i in last_drop..current_idx {
                //     self.debug[i] += 1;
                // }
                // self.debug.push(1);

                // eprintln!(" out: {:?}", self);
            },
        }
    }
}

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut scanner = if let Some(head) = ratings.first() {
            Scanner::init(*head)
        } else {
            return 0
        };

        for (idx, value) in ratings.iter().enumerate().skip(1) {
            scanner.scan(idx, *value);
        }

        scanner.acc as i32
    }
}
