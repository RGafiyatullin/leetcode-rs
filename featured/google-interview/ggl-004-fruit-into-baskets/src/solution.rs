pub struct Solution;

impl Solution {
    pub fn total_fruit(trees: Vec<i32>) -> i32 {
        assert!(trees.len() >= 1);
        assert!(trees.len() <= 100000);

        // eprintln!("INPUT: {:?}", trees);
        trees
            .into_iter()
            // .inspect(|t| eprintln!("SINGLE {}", t))
            .map(Some)
            .chain(std::iter::once(None))
            .scan(Option::<(i32, usize)>::None, |last, fruit_type| {
                match (fruit_type, last.take()) {
                    (Some(fruit_type), Some((same_type, q))) if same_type == fruit_type => {
                        *last = Some((same_type, q + 1));
                        Some(None)
                    },
                    (Some(fruit_type), Some((other_type, other_q))) => {
                        assert_ne!(other_type, fruit_type);

                        *last = Some((fruit_type, 1));
                        Some(Some((other_type, other_q)))
                    },
                    (Some(fruit_type), None) => {
                        *last = Some((fruit_type, 1));
                        Some(None)
                    },
                    (None, None) => Some(None),
                    (None, Some((fruit_type, q))) => Some(Some((fruit_type, q))),
                }
            })
            .filter_map(std::convert::identity)
            // .inspect(|g| eprintln!("GROUP {:?}", g))
            // .fold(Folder::Empty { max: 0 }, |f0, g| {
            //     let f1 = f0.fold(g);
            //     eprintln!("FOLD\n\t{:?} + {:?} =\n\t{:?}", f0, g, f1);
            //     f1
            // })
            .fold(Folder::Empty { max: 0 }, Folder::fold)
            .max() as i32
    }
}

#[derive(Debug, Clone, Copy)]
enum Folder {
    Empty { max: usize },
    One { max: usize, t: i32, c: usize },
    TwoA { max: usize, l_t: i32, l_c: usize, r_t: i32, r_c: usize },
    TwoB { max: usize, l_t: i32, l_c: usize, l_cp: usize, r_t: i32, r_c: usize },
}

impl Folder {
    fn max(&self) -> usize {
        match *self {
            Self::Empty { max, .. } => max,
            Self::One { max, .. } => max,
            Self::TwoA { max, .. } => max,
            Self::TwoB { max, .. } => max,
        }
    }
    fn fold(self, group: (i32, usize)) -> Self {
        let (next_type, next_count) = group;
        match self {
            Self::Empty { max } =>
                Self::One { max: std::cmp::max(max, next_count), t: next_type, c: next_count },

            Self::One { max, t, c } => {
                assert_ne!(t, next_type);
                let total = c + next_count;
                Self::TwoA {
                    max: std::cmp::max(max, total),
                    l_t: next_type,
                    l_c: next_count,
                    r_t: t,
                    r_c: c,
                }
            },

            Self::TwoA { max, l_t, l_c, r_t, r_c } => {
                assert_ne!(l_t, next_type);

                if r_t == next_type {
                    let total = l_c + r_c + next_count;
                    Self::TwoB {
                        max: std::cmp::max(max, total),
                        l_t: r_t,
                        l_c: next_count,
                        l_cp: r_c,
                        r_t: l_t,
                        r_c: l_c,
                    }
                } else {
                    let total = l_c + next_count;
                    Self::TwoA {
                        max: std::cmp::max(max, total),
                        l_t: next_type,
                        l_c: next_count,
                        r_t: l_t,
                        r_c: l_c,
                    }
                }
            },

            Self::TwoB { max, l_t, l_c, l_cp, r_t, r_c } => {
                assert_ne!(l_t, next_type);

                if r_t == next_type {
                    let total = next_count + r_c + l_c + l_cp;
                    Self::TwoB {
                        max: std::cmp::max(max, total),
                        l_t: r_t,
                        l_c: next_count,
                        l_cp: r_c,
                        r_t: l_t,
                        r_c: l_c + l_cp,
                    }
                } else {
                    let total = next_count + l_c;
                    Self::TwoA {
                        max: std::cmp::max(max, total),
                        l_t: next_type,
                        l_c: next_count,
                        r_t: l_t,
                        r_c: l_c,
                    }
                }
            },
        }
    }
}
