use super::*;

pub trait FirstInFirstOutBucket: Bucket {
    fn enqueue(&mut self, item: Self::Item);

    fn dequeue(&mut self) -> Option<Self::Item>;
}

// ---------------------------------------------------------------------------
// Provide a canonical implementation of FirstInFirstOutBucket using VecDeque:
// ---------------------------------------------------------------------------

use std::collections::VecDeque;

impl<T> FirstInFirstOutBucket for VecDeque<T> {
    fn enqueue(&mut self, item: Self::Item) {
        self.push_back(item)
    }

    fn dequeue(&mut self) -> Option<Self::Item> {
        self.pop_front()
    }
}
