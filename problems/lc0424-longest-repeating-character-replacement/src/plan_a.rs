pub struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut max = 0;
        for c in 'A'..='Z' {
            let l = character_replacement_single_letter(c as u8, s.as_bytes(), k as usize);
            // eprintln!("({}) -> {}", c, l);
            max = l.max(max);
        }
        max as i32
    }
}

fn character_replacement_single_letter(c: u8, s: &[u8], k: usize) -> usize {
    let skipped = if let Some(skipped) =
        s.iter().enumerate().find_map(|(i, &l)| if l == c { Some(i) } else { None })
    {
        skipped
    } else {
        return 0
    };

    let mut head = skipped;
    let mut available = k;
    let mut max_len = 0;
    for tail in head..s.len() {
        if s[tail] != c {
            if let Some(left) = available.checked_sub(1) {
                available = left;
            } else {
                for h in head..=tail {
                    if s[h] != c {
                        head = h + 1;
                        break
                    } else {
                        head = h;
                    }
                }
            }
        }
        if tail < head {
            continue
        }

        let len = tail - head + 1;

        // eprint!("{:3} ", tail);
        // eprint!("{:3} ", available);
        // eprint!("{:3} ", head);
        // eprint!("{:3} ", len);
        // eprintln!();

        max_len = max_len.max(len);
    }

    max_len + available.min(skipped)
}
