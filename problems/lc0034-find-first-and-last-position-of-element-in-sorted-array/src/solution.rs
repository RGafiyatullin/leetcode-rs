pub struct Solution;

use std::cmp::Ordering;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_idx = Option::<usize>::None;
        let mut right_idx = Option::<usize>::None;
        let mut tasks = vec![Task::FindAny(Range::new(0, nums.len()))];

        while let Some(task) = tasks.pop() {
            match task {
                Task::FindAny(range) => {
                    if let Some((mid_idx, left, right)) = range.split() {
                        match nums[mid_idx].cmp(&target) {
                            Ordering::Equal => {
                                // eprintln!("[A] Found {:?} at {:?}", target, mid_idx);

                                left_idx = Some(mid_idx);
                                right_idx = Some(mid_idx);

                                tasks.push(Task::FindLeft(left));
                                tasks.push(Task::FindRight(right));
                            }
                            Ordering::Less => {
                                tasks.push(Task::FindAny(right));
                            }
                            Ordering::Greater => {
                                tasks.push(Task::FindAny(left));
                            }
                        }
                    }
                }
                Task::FindLeft(range) => {
                    if let Some((mid_idx, left, right)) = range.split() {
                        match nums[mid_idx].cmp(&target) {
                            Ordering::Equal => {
                                // eprintln!("[L] Found {:?} at {:?}", target, mid_idx);

                                left_idx = Some(mid_idx);

                                tasks.push(Task::FindLeft(left));
                            }
                            Ordering::Less => {
                                tasks.push(Task::FindLeft(right));
                            }
                            Ordering::Greater => {
                                panic!("[L] the array is not sorted");
                            }
                        }
                    }
                }
                Task::FindRight(range) => {
                    if let Some((mid_idx, left, right)) = range.split() {
                        match nums[mid_idx].cmp(&target) {
                            Ordering::Equal => {
                                // eprintln!("[R] Found {:?} at {:?}", target, mid_idx);

                                right_idx = Some(mid_idx);

                                tasks.push(Task::FindRight(right));
                            }
                            Ordering::Greater => {
                                tasks.push(Task::FindRight(left));
                            }
                            Ordering::Less => {
                                panic!("[R] the array is not sorted");
                            }
                        }
                    }
                }
            }
        }

        vec![
            left_idx.map(|i| i as i32).unwrap_or(-1),
            right_idx.map(|i| i as i32).unwrap_or(-1),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
enum Task {
    FindAny(Range),
    FindLeft(Range),
    FindRight(Range),
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
    pub fn middle(&self) -> Option<usize> {
        if !self.is_empty() {
            Some((self.start + self.end) / 2)
        } else {
            None
        }
    }

    pub fn split(&self) -> Option<(usize, Self, Self)> {
        self.middle().map(|mid_idx| {
            (
                mid_idx,
                Self::new(self.start, mid_idx),
                Self::new(mid_idx + 1, self.end),
            )
        })
    }
}
