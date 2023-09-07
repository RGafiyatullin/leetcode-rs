pub struct Solution;

const NUM_MAX: i32 = 10_000;
const NUM_MIN: i32 = -10_000;
const RANGE_SIZE: usize = (NUM_MAX - NUM_MIN + 1) as usize;

type FREQS = [usize; RANGE_SIZE];

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut freqs: FREQS = [0; RANGE_SIZE];
        for n in nums {
            freqs[idx(n)] += 1;
        }

        let mut heap = std::collections::BinaryHeap::new();
        for (idx, freq) in freqs.iter().copied().enumerate() {
            if freq == 0 {
                continue
            }

            let num = num(idx);
            heap.push(HeapEntry { num, freq });
            heap.shrink_to(k);
        }

        let mut out = vec![];
        for _ in 0..k {
            if let Some(entry) = heap.pop() {
                out.push(entry.num);
            }
        }
        out
    }
}

fn idx(n: i32) -> usize {
    let pos = n - NUM_MIN;
    assert!(!pos.is_negative());
    let i = pos as usize;
    i
}

fn num(i: usize) -> i32 {
    let pos = i as i32;
    let n = pos + NUM_MIN;
    n
}

#[derive(Eq, Ord)]
struct HeapEntry {
    num: i32,
    freq: usize,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.freq.eq(&other.freq)
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.freq.partial_cmp(&other.freq)
    }
}
