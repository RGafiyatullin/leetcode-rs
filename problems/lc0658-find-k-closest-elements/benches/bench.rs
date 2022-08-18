#[macro_use]
extern crate bencher;

use bencher::Bencher;

use lc0658_find_k_closest_elements::tests::CASES;

fn do_builtin_sort_by(bench: &mut Bencher) {
    use lc0658_find_k_closest_elements::builtin_sort_by::Solution;
    bench.iter(|| {
        for &(input, k, x, exp) in CASES {
            assert_eq!(Solution::find_closest_elements(input.to_vec(), k, x), exp);
        }
    });
}
fn do_window_and_bisect(bench: &mut Bencher) {
    use lc0658_find_k_closest_elements::window_and_bisect::Solution;
    bench.iter(|| {
        for &(input, k, x, exp) in CASES {
            assert_eq!(Solution::find_closest_elements(input.to_vec(), k, x), exp);
        }
    });
}

benchmark_group!(builtin_sort_by, do_builtin_sort_by);
benchmark_group!(window_and_bisect, do_window_and_bisect);

benchmark_main!(builtin_sort_by, window_and_bisect);
