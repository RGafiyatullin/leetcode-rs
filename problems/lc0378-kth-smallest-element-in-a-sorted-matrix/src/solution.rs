pub struct Solution;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let singles = matrix
            .into_iter()
            .map(MergeSort::single)
            .collect::<Vec<_>>();

        let merge_sorted_opt = {
            let mut doubles = singles;

            while doubles.len() > 1 {
                let mut tmp = vec![];
                while !doubles.is_empty() {
                    let left = doubles.pop().expect("!doubles.is_empty()");
                    if let Some(right) = doubles.pop() {
                        tmp.push(MergeSort::double(left, right));
                    } else {
                        tmp.push(left);
                    }
                }
                std::mem::swap(&mut doubles, &mut tmp);
            }

            doubles.pop()
        };

        merge_sorted_opt
            .expect("Empty matrix")
            .skip(k as usize - 1)
            .next()
            .expect("k is too large")
    }
}

enum MergeSort<Inner> {
    Single(Option<i32>, Inner),
    Double(Box<Self>, Box<Self>),
}

impl<Inner> MergeSort<Inner> {
    pub fn single<I>(root: I) -> Self
    where
        I: IntoIterator<IntoIter = Inner>,
        Inner: Iterator<Item = i32>,
    {
        let mut root = root.into_iter();
        let peeked = root.next();
        Self::Single(peeked, root)
    }

    pub fn double(left: Self, right: Self) -> Self {
        let left = Box::new(left);
        let right = Box::new(right);

        Self::Double(left, right)
    }

    fn peek(&self) -> Option<i32> {
        match self {
            Self::Single(peeked, _inner) => *peeked,
            Self::Double(left, right) => match (left.peek(), right.peek()) {
                (None, None) => None,
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (Some(l), Some(r)) => Some(std::cmp::min(l, r)),
            },
        }
    }
}

impl<Inner> Iterator for MergeSort<Inner>
where
    Inner: Iterator<Item = i32>,
{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Single(peeked, inner) => {
                let out = *peeked;
                *peeked = inner.next();

                out
            }
            Self::Double(left, right) => match (left.peek(), right.peek()) {
                (None, None) => None,
                (Some(_l), None) => left.next(),
                (None, Some(_r)) => right.next(),
                (Some(l), Some(r)) if l <= r => left.next(),
                (Some(l), Some(r)) => {
                    assert!(l > r);
                    right.next()
                }
            },
        }
    }
}
