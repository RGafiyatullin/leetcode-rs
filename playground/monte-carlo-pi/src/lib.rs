use rand::prelude::Distribution;

#[derive(Debug, Clone)]
pub struct Calculator {
    hit: usize,
    miss: usize,
    rng: rand::rngs::ThreadRng,
    dist: rand::distributions::Uniform<f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            hit: 0,
            miss: 0,
            rng: Default::default(),
            dist: rand::distributions::Uniform::new_inclusive(0.0, 1.0),
        }
    }
    pub fn tick(&mut self) {
        let x = self.dist.sample(&mut self.rng);
        let y = self.dist.sample(&mut self.rng);

        if x * x + y * y <= 1.0 {
            self.hit += 1;
        } else {
            self.miss += 1;
        }
    }
    pub fn pi(&self) -> f64 {
        let four = (self.hit + self.miss) as f64;
        let pi = self.hit as f64;

        pi / four * 4.0
    }
}
