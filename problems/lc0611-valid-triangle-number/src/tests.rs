use std::time::Duration;

use crate::linscan::Solution;
// use crate::bisect::Solution;
// use crate::naive::Solution;

const CASES: &[(&[i32], i32)] = &[
    (&[2, 2, 3, 4], 3),
    (&[4, 2, 3, 4], 4),
    (&[1], 0),
    (&[1, 1], 0),
    (&[1, 1, 1], 1),
    (&[1, 2, 2], 1),
    (&[1, 2, 3], 0),
    (&[1, 1, 1, 1], 4),
    (&[1, 2, 3, 4], 1),
    (&[1, 1, 1, 1, 1], 10),
    (&[1, 2, 3, 4, 5], 3),
    (&[1, 1, 1, 1, 1, 1], 20),
    (&[1, 2, 3, 4, 5, 6], 7),
    (&[24, 3, 82, 22, 35, 84, 19], 10),
    (
        &[
            // 3,
            // 19,
            // 22,
            24, 35, 82, 84,
        ],
        2,
    ),
];

#[test]
fn run_all_cases() {
    let mut alright = true;
    CASES
        .into_iter()
        .copied()
        .enumerate()
        .for_each(|(idx, (nums, exp))| {
            eprintln!("#{}: {:?} -> {:?}", idx, nums, exp);

            let act = Solution::triangle_number(nums.to_vec());
            eprintln!(
                "\t{:?} (exp: {:?}) [{}]",
                act,
                exp,
                if act == exp { "OK" } else { "FAIL" }
            );

            alright = alright && act == exp;
        });

    assert!(alright);
}

#[test]
fn run_specific_case() {
    let (nums, _exp) = CASES[12];
    let exp = crate::naive::Solution::triangle_number(nums.to_vec());
    eprintln!("{:?} -> {:?}\n", nums, exp);
    assert_eq!(Solution::triangle_number(nums.to_vec()), exp);
}

#[test]
fn many_sequential() {
    assert_eq!(Solution::triangle_number((1..1000).collect()), 82709749);
}

#[test]
fn many_equal() {
    assert_eq!(Solution::triangle_number(vec![1; 1000]), 166167000);
}

#[test]
fn bench() {
    const TIMES: usize = 10;

    use crate::linscan::{count_lt_bisect, count_lt_scan};

    let data = (0..1_000_000_000).collect::<Vec<_>>();

    std::iter::repeat(()).scan(2, |state, ()| {
        *state *= 2;
        Some(*state)
    }).take_while(|&input_size| {
        if input_size > data.len() {
            return false
        }
        
        let nums = &data[0..input_size];

        let t = std::time::Instant::now();
        let scan_duration = time_it(TIMES, || { count_lt_scan(nums,  t.elapsed().as_secs() as i32 * nums.last().copied().unwrap()); });
        let bisect_duration = time_it(TIMES, || { count_lt_bisect(nums, t.elapsed().as_secs() as i32 * nums.last().copied().unwrap()); });

        eprintln!("INPUT-SIZE: {:?} ({:?} times)", input_size, TIMES);
        eprintln!("  SCAN:   {:?}", scan_duration);
        eprintln!("  BISECT: {:?}", bisect_duration);

        (scan_duration < bisect_duration) || input_size < 4096
    }).count();
}

fn time_it<F: Fn()>(times: usize, func: F) -> Duration {
    let func = bencher::black_box(func);
    let t0 = std::time::Instant::now();
    let mut do_not_ignore = vec![];
    
    for _ in 0..times {
        do_not_ignore.push((func)());
    }
    eprintln!("  {:?} times done", do_not_ignore.len());

    t0.elapsed()
}
