// https://leetcode.com/problems/find-the-town-judge/

fn main() -> () {
    let stdin = std::io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let n: i32 = serde_json::from_str(&input).expect("Failed to parse input as i32");
    input.clear();

    stdin.read_line(&mut input).unwrap();
    let trust: Vec<Vec<i32>> =
        serde_json::from_str(&input).expect("Failed to parse input as Vec<Vec<i32>>");
    input.clear();

    let solution = Solution::find_judge(n, trust);

    println!("solution: {}", solution);
}

struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;

        let n = n as usize;
        let mut trusted = (0..n).map(|_| 0).collect::<Vec<usize>>();
        let mut trusts = (0..n).map(|_| 0).collect::<Vec<usize>>();

        for edge in trust {
            assert_eq!(edge.len(), 2);

            let truster = i2u(edge[0]);
            let trustee = i2u(edge[1]);

            trusted[trustee] = trusted[trustee] + 1;
            trusts[truster] = trusts[truster] + 1;
        }

        // println!("trusted: {:#?}", trusted);
        // println!("trusts: {:#?}", trusts);

        let candidates_1: HashSet<_> = trusts
            .into_iter()
            .enumerate()
            .filter_map(|(id, c)| if c == 0 { Some(id) } else { None })
            .collect();
        let candidates_2: HashSet<_> = trusted
            .into_iter()
            .enumerate()
            .filter_map(|(id, c)| if c == (n - 1) { Some(id) } else { None })
            .collect();

        let judges: Vec<_> = candidates_1.intersection(&candidates_2).collect();

        // println!("candidates-1: {:#?}", candidates_1);
        // println!("candidates-2: {:#?}", candidates_2);
        // println!("judges: {:#?}", judges);

        if judges.len() == 1 {
            u2i(*judges[0])
        } else {
            -1
        }
    }
}

fn i2u(i: i32) -> usize {
    (i - 1) as usize
}
fn u2i(u: usize) -> i32 {
    (u + 1) as i32
}
