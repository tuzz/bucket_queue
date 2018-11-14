use super::*;

pub trait DoubleEndedBucket: Bucket {
    fn push_back(&mut self, item: Self::Item);
    fn push_front(&mut self, item: Self::Item);

    fn pop_back(&mut self) -> Option<Self::Item>;
    fn pop_front(&mut self) -> Option<Self::Item>;
}

// -----------------------------------------------------------------------
// Provide a canonical implementation of DoubleEndedBucket using VecDeque:
// -----------------------------------------------------------------------

use std::collections::VecDeque;

impl<T> DoubleEndedBucket for VecDeque<T> {
    fn push_back(&mut self, item: Self::Item) {
        self.push_back(item)
    }

    fn push_front(&mut self, item: Self::Item) {
        self.push_front(item)
    }

    fn pop_back(&mut self) -> Option<Self::Item> {
        self.pop_back()
    }

    fn pop_front(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}


// -------------------------------------------------------------------
// Implement DoubleEndedBucket for DeferredBucket to support deferral:
// -------------------------------------------------------------------

impl<'a, Q, B> DoubleEndedBucket for DeferredBucket<'a, Q, B>
    where Q: DoubleEndedQueue<B>,
          B: DoubleEndedBucket,
{
    fn push_back(&mut self, item: Self::Item) {
        self.adding().push_back(item)
    }

    fn push_front(&mut self, item: Self::Item) {
        self.adding().push_front(item)
    }

    fn pop_back(&mut self) -> Option<Self::Item> {
        self.removing()?.pop_back()
    }

    fn pop_front(&mut self) -> Option<Self::Item> {
        self.removing()?.pop_front()
    }
}
