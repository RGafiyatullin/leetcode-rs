use std::io::BufRead;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    let mut key = None;
    for line in std::io::stdin().lock().lines() {
        let line = line.expect("Failed to read from stdin");
        if let Some(key) = key.take() {
            let output = solution::Solution::decode_message(key, line);
            eprintln!(">> {:?}", output);
        } else {
            key = Some(line);
        }
    }
}
