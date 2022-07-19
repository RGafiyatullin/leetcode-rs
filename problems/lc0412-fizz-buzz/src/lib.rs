pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..=n).map(|i| match (i % 3 == 0, i % 5 == 0) {
            (true, true) => "FizzBuzz".to_owned(),
            (true, false) => "Fizz".to_owned(),
            (false, true) => "Buzz".to_owned(),
            (false, false) => i.to_string(),
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::fizz_buzz(15), &["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]);
    }
}
