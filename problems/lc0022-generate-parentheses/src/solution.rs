
pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        assert!(n <= 8);
        assert!(n >= 1);

        let mut out = Default::default();
        generate(&mut out, "", n as usize, 0);
        out.into_iter().collect()
    }
}

use std::collections::HashSet;

fn generate(out: &mut HashSet<String>, prefix: &str, to_open: usize, to_close: usize) {
    if let Some(to_close) = to_close.checked_sub(1) {
        generate(out, &format!("{})", prefix), to_open, to_close);
    }
    if let Some(to_open) = to_open.checked_sub(1) {
        generate(out, &format!("{}(", prefix), to_open, to_close + 1);
    }

    if to_open == 0 && to_close == 0 {
        out.insert(prefix.to_owned());
    }
}

