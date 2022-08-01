#[derive(Debug)]
pub struct NumArray {
    nums: Vec<i32>,
    inner: Range,
}

#[derive(Debug)]
struct Range {
    lo_inc: usize,
    hi_exc: usize,

    sum: Option<i32>,
    subs: Option<Subs>,
}

#[derive(Debug)]
struct Subs {
    lo_inc: usize,
    mid: usize,
    hi_exc: usize,

    ready: Option<Box<(Range, Range)>>,
}

impl Subs {
    fn ensure_child_nodes(&mut self) -> (&mut Range, &mut Range) {
        let lo = Range::build(self.lo_inc, self.mid);
        let hi = Range::build(self.mid, self.hi_exc);

        if self.ready.is_none() {
            self.ready = Some(Box::new((lo, hi)));
        }

        self.ready
            .as_mut()
            .map(|pair| (&mut pair.0, &mut pair.1))
            .expect("I have just ensured it is Some")
    }
}

const THRESHOLD: usize = 4;

impl Range {
    fn build(idx_lo_inc: usize, idx_hi_exc: usize) -> Self {
        assert!(idx_lo_inc <= idx_hi_exc);

        let subs = if idx_hi_exc - idx_lo_inc <= THRESHOLD {
            None
        } else {
            Some(Subs {
                lo_inc: idx_lo_inc,
                mid: (idx_hi_exc + idx_lo_inc) / 2,
                hi_exc: idx_hi_exc,
                ready: None,
            })
        };

        Self {
            lo_inc: idx_lo_inc,
            hi_exc: idx_hi_exc,
            sum: None,
            subs,
        }
    }

    fn update(&mut self, index: usize) {
        if index >= self.lo_inc && index < self.hi_exc {
            self.sum = None;
            if let Some(subs) = self.subs.as_mut() {
                subs.ready = None;
            }
        }
    }

    fn sum_range(&mut self, slice: &[i32], left_inc: usize, right_exc: usize) -> i32 {
        eprintln!("sum_range: [{:?}..{:?}) @ [{:?}..{:?})", left_inc, right_exc, self.lo_inc, self.hi_exc);

        if left_inc >= self.hi_exc || right_exc <= self.lo_inc {
            // out of this node's range
            eprintln!("out of range");
            0
        } else {
            // within this node's range

            let lo_inc = std::cmp::max(self.lo_inc, left_inc);
            let hi_exc = std::cmp::min(self.hi_exc, right_exc);

            let complete_range = hi_exc == self.hi_exc && lo_inc == self.lo_inc;

            if complete_range {
                // this range is included completely...
                if let Some(sum) = self.sum {
                    eprintln!("HIT [{:?}, {:?})", self.lo_inc, self.hi_exc);
                    // ... and there is a cached value
                    return sum;
                }
            }

            if let Some(subs) = self.subs.as_mut() {
                let (lo, hi) = subs.ensure_child_nodes();

                let lo_sum = lo.sum_range(slice, left_inc, right_exc);
                let hi_sum = hi.sum_range(slice, left_inc, right_exc);

                if complete_range {
                    self.sum = Some(lo_sum + hi_sum);
                }

                lo_sum + hi_sum
            } else {
                eprintln!("Calculating... [{:?}..{:?}) @ [{:?}..{:?})", lo_inc, hi_exc, self.lo_inc, self.hi_exc);
                // this is a small range — just calculate it
                let total_sum = slice[self.lo_inc..self.hi_exc].iter().copied().sum();
                let range_sum = slice[lo_inc..hi_exc].iter().copied().sum();

                self.sum = Some(total_sum);

                range_sum
            }
        }
    }
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let inner = Range::build(0, nums.len());
        Self { nums, inner }
    }

    pub fn update(&mut self, index: i32, new_val: i32) {
        let index = index as usize;
        self.nums[index] = new_val;
        self.inner.update(index)
    }

    pub fn sum_range(&mut self, left_inc: i32, right_inc: i32) -> i32 {
        let left_inc = left_inc as usize;
        let right_exc = right_inc as usize + 1;

        assert!(left_inc <= right_exc);

        self.inner.sum_range(&self.nums[..], left_inc, right_exc)
    }
}
