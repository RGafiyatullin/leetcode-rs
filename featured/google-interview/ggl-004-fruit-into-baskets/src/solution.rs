pub struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let candidates = Vec::<Candidate>::with_capacity(fruits.len());
        let candidates = fruits.into_iter().fold(candidates, |mut candidates, fruit_type| {
            candidates.push(Candidate::One(fruit_type, 0));
            candidates.iter_mut().for_each(|candidate| {
                *candidate = candidate.visit_fruit(fruit_type)
            });
            candidates
        });
        candidates.iter().map(Candidate::fruit_count).max().unwrap_or(0) as i32
    }
}

#[derive(Debug, Clone, Copy)]
enum Candidate {
    One(i32, usize),
    Two(i32, usize, i32, usize),
    Done(usize),
}

impl Candidate {
    fn fruit_count(&self) -> usize {
        match self {
            Candidate::One(_, count) => *count,
            Candidate::Two(_, count_1, _, count_2) => *count_1 + *count_2,
            Candidate::Done(count) => *count,
        }
    }
    fn visit_fruit(self, fruit_type: i32) -> Self {
        match self {
            Self::One(basket_1, count) if fruit_type == basket_1 =>
                Self::One(basket_1, count + 1),
            Self::One(basket_1, count) => {
                assert_ne!(fruit_type, basket_1);
                Self::Two(basket_1, count, fruit_type, 1)
            },
            Self::Two(basket_1, count_1, basket_2, count_2) if fruit_type == basket_1 =>
                Self::Two(basket_1, count_1 + 1, basket_2, count_2),
            Self::Two(basket_1, count_1, basket_2, count_2) if fruit_type == basket_2 =>
                Self::Two(basket_1, count_1, basket_2, count_2 + 1),
            
            Self::Two(basket_1, count_1, basket_2, count_2) => {
                assert_ne!(fruit_type, basket_1);
                assert_ne!(fruit_type, basket_2);

                Self::Done(count_1 + count_2)
            },
            done @ Self::Done {.. } => done
        }
    }
}