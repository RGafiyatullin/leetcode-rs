use std::io::BufRead;

mod sort_inplace;
mod tree;

fn main() {
    for line in std::io::stdin().lock().lines() {
        let mode = std::env::var("MODE").unwrap_or("TREE".to_owned());

        let line = line.expect("Failed to read line from stdin");
        match line
            .split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<Vec<i32>, _>>()
        {
            Ok(nums) => {
                let out = match mode.as_str() {
                    "TREE" => tree::Solution::longest_consecutive(nums),
                    "SORT-INPLACE" => sort_inplace::Solution::longest_consecutive(nums),
                    _ => panic!("Unknown mode: {}", mode),
                };

                eprintln!("out> {}", out);
            }
            Err(reason) => eprintln!("Failed to parse input: {}", reason),
        }
    }
}
