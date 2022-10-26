// https://leetcode.com/problems/max-points-on-a-line/

fn main() -> () {
    let count = Solution::max_points(vec![
        // vec![1, 1],
        // vec![3, 2],
        // vec![5, 3],
        // vec![4, 1],
        // vec![2, 3],
        // vec![1, 4],
        vec![0, 0],
        vec![0, 0],
    ]);
    println!("{}", count);
}

struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 1 {
            return 1
        }

        use std::collections::{HashMap, HashSet};

        let points = points
            .into_iter()
            .enumerate()
            .filter_map(
                |(id, dims)| {
                    if dims.len() == 2 {
                        Some((dims[0], dims[1], id))
                    } else {
                        None
                    }
                },
            )
            .collect::<Vec<_>>();

        let count = points
            .iter()
            // .enumerate()
            .map(|(x1, y1, id1)| {
                let (x1, y1, id1) = (*x1, *y1, *id1);
                // let idx = idx + 1;
                points.iter().map(move |(x2, y2, id2)| {
                    let (x2, y2, id2) = (*x2, *y2, *id2);
                    ((x1, y1, id1), (x2, y2, id2))
                })
            })
            .flatten()
            .fold(HashMap::<Line, HashSet<usize, _>, _>::new(), |mut acc, (dot1, dot2)| {
                let line = Line::from_dots((dot1.0, dot1.1), (dot2.0, dot2.1));

                if let Some(dots) = acc.get_mut(&line) {
                    dots.insert(dot1.2);
                    dots.insert(dot2.2);
                } else {
                    let mut dots = HashSet::new();
                    dots.insert(dot1.2);
                    dots.insert(dot2.2);
                    acc.insert(line, dots);
                };
                acc
            })
            .into_iter()
            .map(|(_line, dots)| dots.len())
            .fold(0, |acc, c| if acc > c { acc } else { c });

        count as i32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
struct Rational {
    num: i32,
    den: i32,
}
impl Rational {
    pub fn new(num: i32, den: i32) -> Self {
        assert!(den != 0);

        fn abs(n: i32) -> i32 {
            if n < 0 {
                -n
            } else {
                n
            }
        }

        fn gcd_inner(a: i32, b: i32) -> i32 {
            let mut a = a;
            let mut b = b;
            let mut t: i32;

            while b != 0 {
                t = b;
                b = a % b;
                a = t;
            }
            a
        }

        fn gcd(a: i32, b: i32) -> i32 {
            let a = abs(a);
            let b = abs(b);

            if a < b {
                gcd_inner(b, a)
            } else {
                gcd_inner(a, b)
            }
        }

        let gcd = gcd(num, den);
        let num = num / gcd;
        let den = den / gcd;

        Rational { num, den }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Line {
    Und(i32, i32),
    XEq(i32),
    YEq(i32),
    Gen(Rational, Rational),
}
impl Line {
    pub fn from_dots(dot1: (i32, i32), dot2: (i32, i32)) -> Line {
        if dot1 == dot2 {
            Line::Und(dot1.0, dot1.1)
        } else {
            if dot1.0 == dot2.0 {
                Line::XEq(dot1.0)
            } else if dot1.1 == dot2.1 {
                Line::YEq(dot1.1)
            } else {
                let a = Rational::new(dot2.1 - dot1.1, dot2.0 - dot1.0);

                let b = Rational::new(dot2.0 * dot1.1 - dot1.0 * dot2.1, dot2.0 - dot1.0);

                Line::Gen(a, b)
            }
        }
    }
}
