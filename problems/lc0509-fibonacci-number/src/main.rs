use std::io::BufRead;


mod solution;

fn main() {
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("Failed to read from stdin");
        match line.parse::<i32>() {
            Ok(n) => println!("fib({}) = {}", n, solution::Solution::fib(n)),
            Err(reason) => println!("ERR: {}", reason),
        }
        
    }
}
