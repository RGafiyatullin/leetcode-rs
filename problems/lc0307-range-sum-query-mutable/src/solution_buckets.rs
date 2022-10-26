pub struct NumArray {
    nums: Vec<i32>,
    sum_buckets: Vec<i32>,
    bucket_size: usize,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let bucket_size = (nums.len() as f64).sqrt() as usize;

        let buckets_count = if nums.len() % bucket_size == 0 {
            nums.len() / bucket_size
        } else {
            nums.len() / bucket_size + 1
        };

        let sum_buckets = vec![0; buckets_count];

        let mut out = Self { nums, sum_buckets, bucket_size };

        out.rebuild_buckets();
        out
    }

    pub fn rebuild_buckets(&mut self) {
        for (idx, value) in self.nums.iter().copied().enumerate() {
            let bucket_idx = self.bucket_idx(idx);
            self.sum_buckets[bucket_idx] += value;
        }
    }

    pub fn update(&mut self, index: i32, new_val: i32) {
        let index = index as usize;
        let old_val = self.nums[index];

        let bucket_idx = self.bucket_idx(index);
        self.sum_buckets[bucket_idx] -= old_val;
        self.sum_buckets[bucket_idx] += new_val;
        self.nums[index] = new_val;
    }

    pub fn sum_range(&self, left_inc: i32, right_inc: i32) -> i32 {
        let left_inc = left_inc as usize;
        let right_inc = right_inc as usize;

        let bucket_idx_lo = self.bucket_idx(left_inc);
        let bucket_idx_hi = self.bucket_idx(right_inc);

        if bucket_idx_lo == bucket_idx_hi {
            self.nums[left_inc..=right_inc].iter().copied().sum()
        } else {
            let bucket_bounds_lo = self.bucket_boundaries(left_inc);
            let bucket_bounds_hi = self.bucket_boundaries(right_inc);

            let (fine_sum_left, bucket_idx_lo_inc) = if left_inc == bucket_bounds_lo.0 {
                (0, bucket_idx_lo)
            } else {
                (self.nums[left_inc..bucket_bounds_lo.1].iter().copied().sum(), bucket_idx_lo + 1)
            };
            let (fine_sum_right, bucket_idx_hi_inc) = if right_inc == bucket_bounds_hi.1 - 1 {
                (0, bucket_idx_hi)
            } else {
                (self.nums[bucket_bounds_hi.0..=right_inc].iter().copied().sum(), bucket_idx_hi - 1)
            };

            let coarse_sum: i32 =
                self.sum_buckets[bucket_idx_lo_inc..=bucket_idx_hi_inc].iter().copied().sum();

            fine_sum_left + coarse_sum + fine_sum_right
        }
    }

    fn bucket_idx(&self, num_idx: usize) -> usize {
        num_idx / self.bucket_size
    }

    fn bucket_boundaries(&self, num_idx: usize) -> (usize, usize) {
        let bucket_idx = self.bucket_idx(num_idx);
        let lo_inc = bucket_idx * self.bucket_size;
        let hi_exc = std::cmp::min(lo_inc + self.bucket_size, self.nums.len());

        (lo_inc, hi_exc)
    }
}
