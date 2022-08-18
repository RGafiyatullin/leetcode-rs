#[macro_use]
extern crate bencher;

use bencher::Bencher;

use lc1894_find_the_student_that_will_replace_the_chalk::tests::CASES;

fn do_naive(bench: &mut Bencher) {
    use lc1894_find_the_student_that_will_replace_the_chalk::naive::Solution;
    bench.iter(|| {
        for &(input, k, exp) in CASES {
            assert_eq!(Solution::chalk_replacer(input.to_vec(), k), exp);
        }
    });
}
fn do_moddiv(bench: &mut Bencher) {
    use lc1894_find_the_student_that_will_replace_the_chalk::moddiv::Solution;
    bench.iter(|| {
        for &(input, k, exp) in CASES {
            assert_eq!(Solution::chalk_replacer(input.to_vec(), k), exp);
        }
    });
}

benchmark_group!(naive, do_naive);
benchmark_group!(moddiv, do_moddiv);

benchmark_main!(naive, moddiv);
