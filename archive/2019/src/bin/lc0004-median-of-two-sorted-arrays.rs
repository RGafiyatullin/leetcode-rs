// https://leetcode.com/problems/median-of-two-sorted-arrays/

fn main() -> () {
    let stdin = std::io::stdin();
    let mut line_a = String::new();
    let mut line_b = String::new();
    stdin.read_line(&mut line_a).unwrap();
    stdin.read_line(&mut line_b).unwrap();

    let nums1 = line_a
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    let nums2 = line_b
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();

    let result = Solution::find_median_sorted_arrays(nums1, nums2);

    println!("result: {}", result);
}

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let len = nums1.len() + nums2.len();
        if len < 1 {
            panic!("Assertion failed: You may assume nums1 and nums2 cannot be both empty.");
        }

        let mut it = MergeSort::new(nums1, nums2);
        if len % 2 == 0 {
            let lo = it.nth(len / 2 - 1).expect("iterator returned no item [lo]");
            let hi = it.nth(0).expect("iterator returned no item [hi]");

            (lo + hi) as f64 / 2.0
        } else {
            it.nth(len / 2).expect("iterator returned no item [_]") as f64
        }
    }
}

struct MergeSort<I1, I2>
where
    I1: Iterator<Item = i32>,
    I2: Iterator<Item = i32>,
{
    left: Option<(i32, I1)>,
    right: Option<(i32, I2)>,
}

impl<I1, I2> MergeSort<I1, I2>
where
    I1: Iterator<Item = i32>,
    I2: Iterator<Item = i32>,
{
    pub fn new<II1, II2>(left: II1, right: II2) -> Self
    where
        II1: IntoIterator<Item = i32, IntoIter = I1>,
        II2: IntoIterator<Item = i32, IntoIter = I2>,
    {
        let mut left = left.into_iter();
        let mut right = right.into_iter();

        let left = left.next().map(move |item| (item, left));
        let right = right.next().map(move |item| (item, right));

        Self { left, right }
    }
}

impl<I1, I2> Iterator for MergeSort<I1, I2>
where
    I1: Iterator<Item = i32>,
    I2: Iterator<Item = i32>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let left = self.left.take();
        let right = self.right.take();

        match (left, right) {
            (None, None) => None,
            (Some((item, mut iter)), None) => {
                self.left = iter.next().map(move |i| (i, iter));

                Some(item)
            }
            (None, Some((item, mut iter))) => {
                self.right = iter.next().map(move |i| (i, iter));

                Some(item)
            }
            (Some((l_item, mut l_iter)), Some((r_item, mut r_iter))) => {
                if l_item < r_item {
                    self.left = l_iter.next().map(move |i| (i, l_iter));
                    self.right = Some((r_item, r_iter));

                    Some(l_item)
                } else {
                    self.right = r_iter.next().map(move |i| (i, r_iter));
                    self.left = Some((l_item, l_iter));

                    Some(r_item)
                }
            }
        }
    }
}
