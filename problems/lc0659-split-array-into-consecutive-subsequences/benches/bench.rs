#[macro_use]
extern crate bencher;

use bencher::Bencher;

use lc0659_split_array_into_consecutive_subsequences::tests::CASES;

fn do_active_vec(bench: &mut Bencher) {
    use lc0659_split_array_into_consecutive_subsequences::active_vec::Solution;
    bench.iter(|| {
        for &(input, exp) in CASES {
            assert_eq!(Solution::is_possible(input.to_vec()), exp);
        }
    });
}

fn do_active_vec_long(bench: &mut Bencher) {
    use lc0659_split_array_into_consecutive_subsequences::active_vec::Solution;
    bench.iter(|| {
        assert_eq!(
            Solution::is_possible(
                (1..1000)
                    .enumerate()
                    .flat_map(|(idx, n)| std::iter::repeat(n).take(if idx % 5 == 0 {
                        6
                    } else {
                        10
                    }))
                    .collect()
            ),
            true
        );
    });
}

fn do_active_heap(bench: &mut Bencher) {
    use lc0659_split_array_into_consecutive_subsequences::active_heap::Solution;
    bench.iter(|| {
        for &(input, exp) in CASES {
            assert_eq!(Solution::is_possible(input.to_vec()), exp);
        }
    });
}

fn do_active_heap_long(bench: &mut Bencher) {
    use lc0659_split_array_into_consecutive_subsequences::active_heap::Solution;
    bench.iter(|| {
        assert_eq!(
            Solution::is_possible(
                (1..100)
                    .enumerate()
                    .flat_map(|(idx, n)| std::iter::repeat(n).take(if idx % 5 == 0 {
                        6
                    } else {
                        100
                    }))
                    .collect()
            ),
            true
        );
    });
}

benchmark_group!(active_vec, do_active_vec, do_active_vec_long);
benchmark_group!(active_heap, do_active_heap, do_active_heap_long);

benchmark_main!(active_vec, active_heap);
