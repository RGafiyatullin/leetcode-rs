pub struct Solution;

#[cfg(test)]
mod tests;

impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        assert!(n >= 1);
        assert!(n <= 1000);

        let n = n as usize;
        let mut solutions: Vec<bool> = vec![false];

        for n in 1..=n {
            assert_eq!(solutions.len(), n);

            let outcome = (1..n)
                .rev()
                // .inspect(|x| eprintln!("trying {:?}", x))
                .filter(|x| n % *x == 0)
                // .inspect(|x| eprintln!("candidate — {:?}", x))
                .map(|x| n - x)
                // .inspect(|next| eprintln!("next {:?}", next))
                .map(|next| solutions[next])
                // .inspect(|outcome| eprintln!("{:?}", outcome))
                .any(|outcome| !outcome);
            // eprintln!("=== {:?} => {:?}", n, outcome);

            solutions.push(outcome);
        }

        solutions[n]
    }
}
