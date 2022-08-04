use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Debug, Clone, Default)]
pub struct MyCalendar(BTreeSet<Interval>);

impl MyCalendar {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn book(&mut self, start_inc: i32, end_exc: i32) -> bool {
        let requested = Interval { start_inc, end_exc };

        if let Some(_existing) = self.0.get(&requested) {
            // eprintln!("Conflict [requested: {:?}; existing: {:?}]", requested, _existing);
            false
        } else {
            self.0.insert(requested);
            true
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct Interval {
    start_inc: i32,
    end_exc: i32,
}

impl Interval {
    #[cfg(test)]
    pub fn new(start_inc: i32, end_exc: i32) -> Self {
        assert!(start_inc < end_exc);
        Self { start_inc, end_exc }
    }
}

impl PartialEq for Interval {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (l_s, l_e) = (self.start_inc, self.end_exc);
        let (r_s, r_e) = (other.start_inc, other.end_exc);

        assert!(l_s < l_e);
        assert!(r_s < r_e);

        if r_s >= l_e {
            Some(Ordering::Less)
        } else if l_s >= r_e {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("PartialOrd returned None")
    }
}

impl Eq for Interval {}
