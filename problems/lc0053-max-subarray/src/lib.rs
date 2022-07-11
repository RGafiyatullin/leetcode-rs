mod brute_force;
pub use brute_force::Solution as BruteForce;

mod kadane;
pub use kadane::Solution;

#[cfg(test)]
mod tests;
