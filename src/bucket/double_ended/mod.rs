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
