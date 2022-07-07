pub struct Solution;

impl Solution {
    pub fn is_interleave(left: String, right: String, target: String) -> bool {
        left.chars().for_each(|c| assert!(c.is_ascii_alphabetic()));
        right.chars().for_each(|c| assert!(c.is_ascii_alphabetic()));
        target
            .chars()
            .for_each(|c| assert!(c.is_ascii_alphabetic()));

        let left = left.as_bytes();
        let right = right.as_bytes();
        let target = target.as_bytes();

        let mut candidates: Vec<_> = vec![Candidate {
            src: [Cursor::new(left), Cursor::new(right)],
            target: Cursor::new(target),
        }];

        let mut found = false;
        while let Some(candidate) = candidates.pop() {
            if candidate.check(&mut candidates) {
                found = true;
                break;
            }
        }

        found
    }
}

#[derive(Debug, Clone, Copy)]
struct Cursor<'a> {
    src: &'a [u8],
    cursor: usize,
}
impl<'a> Cursor<'a> {
    fn new(src: &'a [u8]) -> Self {
        Self { src, cursor: 0 }
    }
    fn finished(&self) -> bool {
        self.cursor >= self.src.len()
    }
    fn current(&self) -> Option<u8> {
        self.src.get(self.cursor).copied()
    }
    fn advance(&self) -> Self {
        if !self.finished() {
            Self {
                cursor: self.cursor + 1,
                ..(*self)
            }
        } else {
            panic!("Attempt to advance a finished cursor: {:?}", self)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate<'a, const I: usize> {
    src: [Cursor<'a>; I],
    target: Cursor<'a>,
}

impl<'a, const I: usize> Candidate<'a, I> {
    fn check<C>(&self, candidates: &mut C) -> bool
    where
        C: Extend<Self>,
    {
        if let Some(current) = self.target.current() {
            for (idx, c) in self.src.iter().enumerate() {
                if c.current() == Some(current) {
                    let mut src = self.src;
                    src[idx] = c.advance();
                    candidates.extend(std::iter::once(Self {
                        src,
                        target: self.target.advance(),
                    }));
                }
            }
            false
        } else {
            assert!(self.target.finished());

            self.src.iter().all(|c| c.finished())
        }
    }
}
