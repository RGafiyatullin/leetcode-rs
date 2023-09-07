#![allow(unused)]

pub struct MinStack(Vec<(i32, i32)>);

impl MinStack {
    fn new() -> Self {
        Self(Default::default())
    }
    fn push(&mut self, val: i32) {
        if let Some((_, min)) = self.0.last() {
            let max = val.min(*min);
            self.0.push((val, max));
        } else {
            self.0.push((val, val));
        }
    }
    fn pop(&mut self) {
        self.0.pop().expect(
            "Methods pop, top and getMin operations will always be called on non-empty stacks.",
        );
    }
    fn top(&self) -> i32 {
        self.0
            .last()
            .expect(
                "Methods pop, top and getMin operations will always be called on non-empty stacks.",
            )
            .0
    }
    fn get_min(&self) -> i32 {
        self.0
            .last()
            .expect(
                "Methods pop, top and getMin operations will always be called on non-empty stacks.",
            )
            .1
    }
}
