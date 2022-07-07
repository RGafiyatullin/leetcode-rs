
pub struct Solution;

impl Solution {
    pub fn is_interleave(left: String, right: String, target: String) -> bool {
        left.chars().for_each(|c| assert!(c.is_ascii_alphabetic()));
        right.chars().for_each(|c| assert!(c.is_ascii_alphabetic()));
        target.chars().for_each(|c| assert!(c.is_ascii_alphabetic()));

        let left = left.as_bytes();
        let right = right.as_bytes();
        let target = target.as_bytes();

        

        let mut candidates: Vec<_> = vec![Candidate {
            src: [Cursor::new(left), Cursor::new(right)],
            target: Cursor::new(target),
        }];

        let mut found = false;
        while let Some(mut candidate) = candidates.pop() {
            if candidate.check(&mut candidates) {
                found = true;
                break
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
        Self { src, cursor: 0, }
    }
    fn is_finished(&self) -> bool {
        self.cursor >= self.src.len()
    }
}

#[derive(Debug, Clone, Copy)]
struct Candidate<'a, const I: usize> {
    src: [Cursor<'a>; I],

    target: Cursor<'a>,
}

impl<'a, const I: usize> Candidate<'a, I> {
    fn check<C>(&mut self, candidates: &mut C) -> bool
    where C: Extend<Self>
    {
        if self.target.is_finished() {
            return self.src.iter().all(|c| c.is_finished())
        }
        
        
        unimplemented!()
    }
}

