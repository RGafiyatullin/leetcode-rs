pub struct NumArray {
    nums: Vec<i32>,
    sum_buckets: Vec<i32>,
}

const BUCKET_SIZE: usize = 256;

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        let buckets_count = if nums.len() % BUCKET_SIZE == 0 {
            nums.len() / BUCKET_SIZE
        } else {
            nums.len() / BUCKET_SIZE + 1
        };

        let mut sum_buckets = vec![0; buckets_count];

        for (idx, value) in nums.iter().copied().enumerate() {
            sum_buckets[bucket_idx(idx)] += value;
        }

        Self { nums, sum_buckets }
    }

    pub fn update(&mut self, index: i32, new_val: i32) {
        let index = index as usize;
        let old_val = self.nums[index];

        let bucket_idx = bucket_idx(index);
        self.sum_buckets[bucket_idx] -= old_val;
        self.sum_buckets[bucket_idx] += new_val;
        self.nums[index] = new_val;
    }

    pub fn sum_range(&self, left_inc: i32, right_inc: i32) -> i32 {
        // eprintln!("BUCKETS: {:?}", self.sum_buckets);
        // eprintln!("NUMS: {:?}", self.nums);

        let left_inc = left_inc as usize;
        let right_inc = right_inc as usize;

        let bucket_idx_lo = bucket_idx(left_inc);
        let bucket_idx_hi = bucket_idx(right_inc);

        // eprintln!("left_inc: {:?}", left_inc);
        // eprintln!("bucket_idx_lo: {:?}", bucket_idx_lo);
        // eprintln!("right_inc: {:?}", right_inc);
        // eprintln!("bucket_idx_hi: {:?}", bucket_idx_hi);

        if bucket_idx_lo == bucket_idx_hi {
            self.nums[left_inc..=right_inc].iter().copied().sum()
        } else {
            let bucket_bounds_lo = bucket_boundaries(left_inc, self.nums.len());
            let bucket_bounds_hi = bucket_boundaries(right_inc, self.nums.len());

            // eprintln!("bucket_bounds_lo: {:?}", bucket_bounds_lo);
            // eprintln!("bucket_bounds_hi: {:?}", bucket_bounds_hi);

            let (fine_sum_left, bucket_idx_lo_inc) = if left_inc == bucket_bounds_lo.0 {
                (0, bucket_idx_lo)
            } else {
                (
                    self.nums[left_inc..bucket_bounds_lo.1]
                        .iter()
                        .copied()
                        .sum(),
                    bucket_idx_lo + 1,
                )
            };
            let (fine_sum_right, bucket_idx_hi_inc) = if right_inc == bucket_bounds_hi.1 - 1 {
                (0, bucket_idx_hi)
            } else {
                (
                    self.nums[bucket_bounds_hi.0..=right_inc]
                        .iter()
                        .copied()
                        .sum(),
                    bucket_idx_hi - 1,
                )
            };

            // eprintln!("fine_sum_left: {:?}", fine_sum_left);
            // eprintln!("bucket_idx_lo_inc: {:?}", bucket_idx_lo_inc);

            // eprintln!("fine_sum_right: {:?}", fine_sum_right);
            // eprintln!("bucket_idx_hi_inc: {:?}", bucket_idx_hi_inc);

            let coarse_sum: i32 = self.sum_buckets[bucket_idx_lo_inc..=bucket_idx_hi_inc]
                .iter()
                .copied()
                .sum();

            fine_sum_left + coarse_sum + fine_sum_right
        }
    }
}

fn bucket_idx(num_idx: usize) -> usize {
    num_idx / BUCKET_SIZE
}

fn bucket_boundaries(num_idx: usize, nums_len: usize) -> (usize, usize) {
    let bucket_idx = bucket_idx(num_idx);
    let lo_inc = bucket_idx * BUCKET_SIZE;
    let hi_exc = std::cmp::min(lo_inc + BUCKET_SIZE, nums_len);

    (lo_inc, hi_exc)
}
