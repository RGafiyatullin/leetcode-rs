pub struct Solution;

use std::collections::HashSet;
use std::hash::Hash;

impl Solution {
    pub fn is_interleave(left: String, right: String, target: String) -> bool {
        // left.chars().for_each(|c| assert!(c.is_ascii_alphabetic()));
        // right.chars().for_each(|c| assert!(c.is_ascii_alphabetic()));
        // target
        //     .chars()
        //     .for_each(|c| assert!(c.is_ascii_alphabetic()));

        let left = left.as_bytes();
        let right = right.as_bytes();
        let target = target.as_bytes();

        let mut candidates = CandidatePool::new();
        candidates.extend(std::iter::once(Candidate {
            src: [Cursor::new(left), Cursor::new(right)],
            target: Cursor::new(target),
        }));

        let mut found = false;
        while let Some(candidate) = candidates.next() {
            // eprintln!("candidate: {}", candidate);
            if candidate.check(&mut candidates) {
                found = true;
                break
            }
        }

        found
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Cursor<'a> {
    src: &'a [u8],
    pos: usize,
}
impl<'a> Cursor<'a> {
    fn new(src: &'a [u8]) -> Self {
        Self { src, pos: 0 }
    }
    fn finished(&self) -> bool {
        self.pos >= self.src.len()
    }
    fn current(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }
    fn advance(&self) -> Self {
        if !self.finished() {
            Self { pos: self.pos + 1, ..(*self) }
        } else {
            panic!("Attempt to advance a finished cursor: {:?}", self)
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
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
                    candidates.extend(std::iter::once(Self { src, target: self.target.advance() }));
                }
            }
            false
        } else {
            assert!(self.target.finished());

            self.src.iter().all(|c| c.finished())
        }
    }
}

#[derive(Default, Debug, Clone)]
struct CandidatePool<T> {
    set: HashSet<T>,
    pool: Vec<T>,
}

impl<T> Iterator for CandidatePool<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.pool.pop()
    }
}

impl<T> CandidatePool<T> {
    pub fn new() -> Self {
        Self { set: Default::default(), pool: Default::default() }
    }
}

impl<T> Extend<T> for CandidatePool<T>
where
    T: Hash + Eq + Clone,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            if !self.set.contains(&item) {
                self.pool.push(item.to_owned());
                self.set.insert(item);
            }
        }
    }
}

impl PartialEq for Cursor<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Hash for Cursor<'_> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl<'a, const I: usize> std::fmt::Display for Candidate<'a, I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Candidate[target: {}; src: ", self.target.pos)?;
        self.src.iter().fold(&mut f.debug_list(), |l, s| l.entry(&s.pos)).finish()?;
        write!(f, "]")?;

        Ok(())
    }
}
