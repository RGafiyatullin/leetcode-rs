use crate::{Interval, MyCalendar};

#[test]
fn test_interval_cmp() {
    assert!(Interval::new(10, 20).cmp(&Interval::new(10, 20)).is_eq());
    assert!(Interval::new(10, 20).cmp(&Interval::new(15, 25)).is_eq());
    assert!(Interval::new(15, 25).cmp(&Interval::new(10, 20)).is_eq());

    assert!(Interval::new(10, 20).cmp(&Interval::new(20, 30)).is_lt());
    assert!(Interval::new(10, 20).cmp(&Interval::new(25, 35)).is_lt());

    assert!(Interval::new(20, 30).cmp(&Interval::new(10, 20)).is_gt());
    assert!(Interval::new(25, 35).cmp(&Interval::new(10, 20)).is_gt());
}

#[test]
fn test_00() {
    with_calendar(|c| {
        ok(c, 10, 20);
        fail(c, 15, 25);
        ok(c, 20, 30);
    });
}

#[test]
fn test_01_no_conflicts_no_overlaps() {
    with_calendar(|c| {
        ok(c, 10, 20);
        ok(c, 30, 40);
    })
}

#[test]
fn test_01_no_conflicts_adjacent() {
    with_calendar(|c| {
        ok(c, 10, 20);
        ok(c, 20, 30);
    })
}

#[test]
fn test_02_start_in_the_middle_of_existing() {
    with_calendar(|c| {
        ok(c, 10, 20);
        fail(c, 15, 25);
    })
}

#[test]
fn test_03_end_in_the_middle_of_existing() {
    with_calendar(|c| {
        ok(c, 10, 20);
        fail(c, 5, 15);
    });
}

#[test]
fn test_04_other_start_in_the_middle() {
    with_calendar(|c| {
        ok(c, 10, 20);
        fail(c, 5, 25);
    });
}

mod utils {
    use super::*;

    pub(super) fn with_calendar<F>(f: F)
    where
        F: FnOnce(&mut MyCalendar),
    {
        let mut c = MyCalendar::new();
        f(&mut c);
        eprintln!("{:#?}", c);
    }

    pub(super) fn ok(c: &mut MyCalendar, start_inc: i32, end_exc: i32) {
        assert_eq!(c.book(start_inc, end_exc), true);
    }
    pub(super) fn fail(c: &mut MyCalendar, start_inc: i32, end_exc: i32) {
        assert_eq!(c.book(start_inc, end_exc), false);
    }
}
use utils::*;
