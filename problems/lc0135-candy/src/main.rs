use std::io::BufRead;
use std::num::ParseIntError;
use std::str::FromStr;

mod local_extrema;

#[cfg(test)]
mod tests;

fn main() {
    for line in std::io::stdin().lock().lines() {
        match process_line(line.expect("Failed to read line").as_str()) {
            Ok(result) => println!("Ok: {:?}", result),
            Err(reason) => println!("Err: {:?}", reason),
        }
    }
}

fn process_line(line: &str) -> Result<i32, ParseIntError> {
    let numbers = line
        .split_whitespace()
        .map(FromStr::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let n = crate::local_extrema::Solution::candy(numbers);

    Ok(n)
}
