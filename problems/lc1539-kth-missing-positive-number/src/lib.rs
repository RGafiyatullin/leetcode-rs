pub struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: impl AsRef<[i32]>, k: i32) -> i32 {
        use std::cmp::Ordering::*;

        let arr = arr.as_ref();

        assert!(arr.len() >= 1);
        assert!(arr.len() <= 1000);
        assert!(k >= 1);
        assert!(k <= 1000);

        let (mut lo, mut hi) = (0, arr.len() - 1);

        let (idx, back, correction) = loop {
            let idx = (lo + hi) / 2;

            // eprintln!(" [{:?}]={:?} ({:?})", idx, arr[idx], metric(idx, arr[idx]));

            let m_prev = idx.checked_sub(1).map(|i| metric(i, arr[i]));
            let m_this = metric(idx, arr[idx]);

            match (lo == hi, m_this.cmp(&k), Some(m_this) == m_prev) {
                (false, Less, _) => lo = idx + 1,
                (false, Greater, _) => hi = idx,
                (_, Equal, false) => break (idx, true, m_this - k),
                (_, Equal, true) => hi = idx,

                (true, Greater, _) => break (idx, true, m_this - k),
                (true, Less, _) => break (idx, false, k - m_this),
            }
        };

        if back {
            arr[idx] - 1 - correction
        } else {
            arr[idx] + correction
        }
    }
}

fn metric(idx: usize, value: i32) -> i32 {
    value - (idx as i32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metric() {
        assert_eq!(metric(0, 1), 0);
        assert_eq!(metric(1, 2), 0);
        assert_eq!(metric(2, 3), 0);
        assert_eq!(metric(3, 4), 0);
        assert_eq!(metric(3, 5), 1);
        assert_eq!(metric(3, 6), 2);
        assert_eq!(metric(3, 7), 3);
        assert_eq!(metric(3, 8), 4);
        assert_eq!(metric(3, 9), 5);
        assert_eq!(metric(3, 10), 6);
        assert_eq!(metric(4, 11), 6);
    }

    const CASES: &[(&[i32], i32, i32)] = &[
        // (&[2, 3, 4, 7, 11], 5, 9),
        // (&[1, 2, 3, 4], 2, 6),
        // (&[1, 3, 5, 7, 9, 11], 1, 2),
        // (&[1, 3, 5, 7, 9, 11], 2, 4),
        // (&[1, 3, 5, 7, 9, 11], 3, 6),
        // (&[1, 3, 5, 7, 9, 11], 4, 8),
        // (&[1, 3, 5, 7, 9, 11], 5, 10),
        // (&[1, 3, 5, 7, 9, 11], 6, 12),
        // (&[1, 3, 5, 7, 9, 11], 7, 13),
        // (&[1, 3, 5, 7, 9, 11], 8, 14),
        (
            &[
                6, 7, 10, 20, 28, 29, 33, 37, 39, 40, 49, 53, 55, 72, 75, 76, 85, 87, 88, 94, 106,
                107, 119, 120, 129, 142, 147, 152, 157, 159, 161, 173, 178, 183, 187, 188, 193,
                199, 202, 212, 215, 224, 227, 230, 237, 239, 246, 251, 256, 260, 266, 268, 273,
                277, 279, 281, 291, 297, 298, 310, 312, 314, 315, 321, 324, 326, 329, 341, 342,
                348, 355, 367, 370, 374, 387, 389, 394, 413, 420, 424, 429, 446, 447, 458, 460,
                464, 467, 473, 477, 478, 498, 500, 501, 503, 514, 515, 523, 525, 528, 529, 531,
                535, 539, 555, 566, 569, 572, 583, 588, 591, 596, 602, 604, 605, 606, 610, 611,
                616, 619, 622, 623, 631, 632, 640, 642, 645, 647, 661, 680, 684, 685, 687, 694,
                696, 698, 714, 717, 720, 726, 734, 738, 742, 745, 753, 770, 775, 780, 781, 783,
                787, 788, 798, 806, 821, 835, 852, 865, 873, 888, 897, 926, 932, 935, 939, 945,
                956, 966, 967, 969, 973, 979, 980, 986, 992, 995, 997,
            ],
            96,
            118,
        ),
    ];

    #[test]
    fn test_solution() {
        for &(input, k, exp) in CASES {
            eprintln!("{:?} | {:?} => {:?}", input, k, exp);
            for (idx, &val) in input.iter().enumerate() {
                eprintln!(" - [{:?}] = {:?} | m={:?}", idx, val, metric(idx, val));
            }
            assert_eq!(Solution::find_kth_positive(input, k), exp);
        }
    }
}
