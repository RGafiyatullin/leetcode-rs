use super::QItem;

#[derive(Debug, Clone)]
pub struct Queue<T> {
    vec: Vec<QItem<T>>,
    head: Option<usize>,
    tail: Option<usize>,
}

impl<T> AsRef<[QItem<T>]> for Queue<T> {
    fn as_ref(&self) -> &[QItem<T>] {
        self.vec.as_ref()
    }
}
impl<T> AsMut<[QItem<T>]> for Queue<T> {
    fn as_mut(&mut self) -> &mut [QItem<T>] {
        self.vec.as_mut()
    }
}

impl<T> Queue<T> {
    pub fn new(vec: Vec<QItem<T>>) -> Self {
        Self {
            vec,
            head: None,
            tail: None,
        }
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    // pub fn is_empty(&self) -> bool {
    //     self.assert_head_and_tail();

    //     self.head.is_none()
    // }

    pub fn unlink_tail(&mut self) -> Option<(usize, Option<usize>)> {
        let idx = self.tail?;
        let (prev, next) = self.unlink(idx);
        assert!(next.is_none());
        Some((idx, prev))
    }

    pub fn unlink(&mut self, idx: usize) -> (Option<usize>, Option<usize>) {
        let prev = self.as_mut()[idx].prev.take();
        let next = self.as_mut()[idx].next.take();

        if let Some(prev) = prev {
            self.as_mut()[prev].next = next;
        } else {
            self.head = next;
        }

        if let Some(next) = next {
            self.as_mut()[next].prev = prev;
        } else {
            self.tail = prev;
        }

        self.assert_head_and_tail();

        (prev, next)
    }

    pub fn push(&mut self, data: T) -> usize {
        let idx = self.vec.len();
        self.vec.push(QItem::new(data));
        idx
    }

    pub fn insert_after(&mut self, after: usize, inserted: usize) {
        assert_ne!(after, inserted);

        self.assert_head_and_tail();
        self.as_ref()[inserted].assert_unlinked();

        self.as_mut()[inserted].prev = Some(after);
        let next = self.as_mut()[after].next.replace(inserted);

        if let Some(next) = next {
            self.as_mut()[next].prev = Some(inserted);
            self.as_mut()[inserted].next = Some(next);
        } else {
            self.tail = Some(inserted);
        }

        self.assert_head_and_tail();
    }

    pub fn insert_before(&mut self, before: usize, inserted: usize) {
        assert_ne!(before, inserted);

        self.assert_head_and_tail();
        self.as_ref()[inserted].assert_unlinked();

        self.as_mut()[inserted].next = Some(before);
        let prev = self.as_mut()[before].prev.replace(inserted);

        if let Some(prev) = prev {
            self.as_mut()[prev].next = Some(inserted);
            self.as_mut()[inserted].prev = Some(prev);
        } else {
            self.head = Some(inserted);
        }

        self.assert_head_and_tail();
    }

    pub fn insert_last(&mut self, inserted: usize) {
        self.assert_head_and_tail();
        self.as_ref()[inserted].assert_unlinked();

        if let Some(tail) = self.tail {
            self.insert_after(tail, inserted);
        } else {
            self.head = Some(inserted);
            self.tail = Some(inserted);
        }

        self.assert_head_and_tail();
    }

    // pub fn insert_first(&mut self, inserted: usize) {
    //     self.assert_head_and_tail();
    //     self.as_ref()[inserted].assert_unlinked();

    //     if let Some(head) = self.head {
    //         self.insert_before(head, inserted);
    //     } else {
    //         self.head = Some(inserted);
    //         self.tail = Some(inserted);
    //     }

    //     self.assert_head_and_tail();
    // }
}

impl<T> Queue<T> {
    fn assert_head_and_tail(&self) {
        assert_eq!(self.head.is_none(), self.tail.is_none());
    }
}
