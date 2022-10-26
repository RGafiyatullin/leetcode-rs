#[macro_use]
extern crate bencher;

use bencher::Bencher;

use lc0871_minimum_number_of_refueling_stops::tests::CASES;

fn do_solution(bench: &mut Bencher) {
    use lc0871_minimum_number_of_refueling_stops::solution::Solution;
    bench.iter(|| {
        for &(target, start_fuel, stations, expected) in CASES {
            eprintln!("{:?} | {:?} @ {:?} -> {:?}", stations, target, start_fuel, expected);
            let actual = Solution::min_refuel_stops(
                target,
                start_fuel,
                stations.into_iter().copied().map(ToOwned::to_owned).collect(),
            );
            assert_eq!(actual, expected);
        }
    });
}

benchmark_group!(solution, do_solution);

benchmark_main!(solution);
