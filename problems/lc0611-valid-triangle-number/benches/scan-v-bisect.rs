#[macro_use]
extern crate bencher;

use bencher::Bencher;

const SIZE_SMALL: i32 = 100;
const SIZE_MEDIUM: i32 = 1_000;
const SIZE_LARGE: i32 = 100_000;

use lc0611_valid_triangle_number::linscan::{count_lt_bisect, count_lt_scan};

fn bisect_small(bench: &mut Bencher) {
    let nums = (1..SIZE_SMALL).collect::<Vec<_>>();
    bench.iter(|| {
        let _ = count_lt_bisect(&nums[..], nums.last().copied().unwrap());
    });
}

fn scan_small(bench: &mut Bencher) {
    let nums = (1..SIZE_SMALL).collect::<Vec<_>>();
    bench.iter(|| {
        let _ = count_lt_scan(&nums[..], nums.last().copied().unwrap());
    })
}

fn bisect_medium(bench: &mut Bencher) {
    let nums = (1..SIZE_MEDIUM).collect::<Vec<_>>();
    bench.iter(|| {
        let _ = count_lt_bisect(&nums[..], nums.last().copied().unwrap());
    });
}

fn scan_medium(bench: &mut Bencher) {
    let nums = (1..SIZE_MEDIUM).collect::<Vec<_>>();
    bench.iter(|| {
        let _ = count_lt_scan(&nums[..], nums.last().copied().unwrap());
    })
}


fn bisect_large(bench: &mut Bencher) {
    let nums = (1..SIZE_LARGE).collect::<Vec<_>>();
    bench.iter(|| {
        let _ = count_lt_bisect(&nums[..], nums.last().copied().unwrap());
    });
}

fn scan_large(bench: &mut Bencher) {
    let nums = (1..SIZE_LARGE).collect::<Vec<_>>();
    bench.iter(|| {
        let _ = count_lt_scan(&nums[..], nums.last().copied().unwrap());
    })
}

benchmark_group!(small, bisect_small, scan_small);
benchmark_group!(medium, bisect_medium, scan_medium);
benchmark_group!(large, bisect_large, scan_large);

benchmark_main!(small, medium, large);