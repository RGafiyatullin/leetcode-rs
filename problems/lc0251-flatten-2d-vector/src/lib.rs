pub struct Vector2D {
    inner: Box<dyn Iterator<Item = i32>>,
    next: Option<i32>,
}

impl Vector2D {
    pub fn new(vec: Vec<Vec<i32>>) -> Self {
        let inner = vec.into_iter().flatten();
        let mut inner = Box::new(inner);
        let next = inner.next();

        Self { inner, next }
    }

    pub fn next(&mut self) -> i32 {
        let item = self.next.take().expect("Only valid calls are expected");
        self.next = self.inner.next();
        item
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
}
