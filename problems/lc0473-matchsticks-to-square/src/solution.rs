pub struct Solution;

impl Solution {
    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        assert!(matchsticks.len() >= 1);
        assert!(matchsticks.len() <= 15);

        let perimeter = matchsticks.iter().sum::<i32>();
        if perimeter % 4 != 0 {
            return false;
        }
        let side_len = perimeter / 4;

        matchsticks.sort();
        matchsticks.reverse();

        solve(
            &Args {
                side_len,
                matchsticks: matchsticks.as_ref(),
            },
            UseMap::new(matchsticks.len() as u8),
            0,
            &mut Default::default(),
        )
    }
}

fn solve(args: &Args, use_map: UseMap, mut sides_done: u8, memo: &mut Memo) -> bool {
    let memo_key = (use_map, sides_done);

    let total = use_map
        .bits_set()
        .map(|idx| args.matchsticks[idx])
        .sum::<i32>();

    if total > 0 && total % args.side_len == 0 {
        sides_done += 1;
    }

    if sides_done == 3 {
        return true;
    }

    if let Some(visited) = memo.get(&memo_key) {
        return *visited;
    }

    let mut ans = false;
    let c = total / args.side_len;
    let rem = args.side_len * (c + 1) - total;

    for idx in use_map.bits_unset() {
        let candidate = args.matchsticks[idx];

        if candidate <= rem {
            if solve(args, use_map.with_bit(idx), sides_done, memo) {
                ans = true;
                break;
            }
        }
    }

    memo.insert(memo_key, ans);

    return ans;
}

type Memo = std::collections::HashMap<(UseMap, u8), bool>;

struct Args<'a> {
    side_len: i32,
    matchsticks: &'a [i32],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UseMap {
    total: u8,
    bits: u16,
}

impl UseMap {
    pub fn new(total: u8) -> Self {
        Self {
            total,
            bits: 0b_0000_0000_0000_0000,
        }
    }

    pub fn bits_set(&self) -> impl Iterator<Item = usize> {
        let bits = self.bits;
        (0..self.total as usize).filter(move |&i| bits & (1 << i) != 0)
    }
    pub fn bits_unset(&self) -> impl Iterator<Item = usize> {
        let bits = self.bits;
        (0..self.total as usize).filter(move |&i| bits & (1 << i) == 0)
    }

    pub fn with_bit(&self, idx: usize) -> Self {
        let bits = self.bits | (1 << idx);
        Self { bits, ..*self }
    }
}
