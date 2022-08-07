pub struct Solution;

const MIN_VAL: i32 = -10000;
const MAX_VAL: i32 = 10000;
const VALUES_COUNT: usize = (MAX_VAL - MIN_VAL + 1) as usize;
const OFFSET: i32 = -MIN_VAL;

impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        assert!(arr.len() >= 2);
        assert!(arr.len() <= 500);

        let mut map = [0 as usize; VALUES_COUNT];
        for &val in &arr {
            assert!(val <= MAX_VAL);
            assert!(val >= MIN_VAL);

            map[index(val)] += 1;
        }

        
        for val in arr {
            if val == 0 {
                if map[index(0)] > 1 {
                    return true
                }
            } else if val >= MIN_VAL && val <= MAX_VAL {
                if map[index(val * 2)] > 0 {
                    return true
                }
            }
        }
        return false
    }
}

fn index(v: i32) -> usize {
    (v + OFFSET) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::check_if_exist([10,2,5,3].to_vec()), true);
        assert_eq!(Solution::check_if_exist([7,1,14,11].to_vec()), true);
        assert_eq!(Solution::check_if_exist([3,1,7,11].to_vec()), false);
        assert_eq!(Solution::check_if_exist([-2,0,10,-19,4,6,-8].to_vec()), false);
        assert_eq!(Solution::check_if_exist([0,0].to_vec()), true);
    }
}
