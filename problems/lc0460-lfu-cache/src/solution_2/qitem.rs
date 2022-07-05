#[derive(Debug, Clone)]
pub struct QItem<T> {
    pub(super) data: T,
    pub(super) prev: Option<usize>,
    pub(super) next: Option<usize>,
}

impl<T> QItem<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            prev: None,
            next: None,
        }
    }

    pub(super) fn assert_unlinked(&self) {
        assert!(self.prev.is_none());
        assert!(self.next.is_none());
    }
}
