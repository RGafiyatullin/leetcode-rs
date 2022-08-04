// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

pub struct Solution {
    total: i32,
    bad_ver: i32,
}
impl Solution {
    #[allow(dead_code)]
    fn new(total: i32, bad_ver: i32) -> Self {
        assert!(bad_ver <= total);
        Self { bad_ver, total }
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        assert!(version <= self.total);

        version >= self.bad_ver
    }
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        assert!(n >= 1);

        let n = n as i64;

        let (mut lo, mut hi) = (1, n - 1);

        let out = loop {
            let ver_this = (lo + hi) / 2;
            let ver_next = ver_this + 1;

            let this_is_bad = self.isBadVersion(ver_this as i32);
            let next_is_bad = self.isBadVersion(ver_next as i32);

            match (lo == hi, this_is_bad, next_is_bad) {
                (_, true, false) => panic!("miracle?"),
                (_, false, true) => break ver_next,

                (true, false, false) => panic!("no bad version"),
                (true, true, true) => {
                    assert_eq!(ver_this, 1);
                    break 1;
                }

                (false, true, true) => hi = ver_this,
                (false, false, false) => lo = ver_this + 1,
            }
        };

        out as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_generated() {
        for total in 1..=10 {
            for bad_ver in 1..=total {
                eprintln!("{:?}/{:?}", bad_ver, total);
                assert_eq!(
                    Solution::new(total, bad_ver).first_bad_version(total),
                    bad_ver
                );
            }
        }
    }

    #[test]
    fn test_01() {
        assert_eq!(
            Solution::new(2126753390, 1702766719).first_bad_version(2126753390),
            1702766719
        )
    }
}
