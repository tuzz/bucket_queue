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


// -----------------------------------------------------------------------
// Implement FirstInFirstOutBucket for DeferredBucket to support deferral:
// -----------------------------------------------------------------------

impl<'a, Q, B> FirstInFirstOutBucket for DeferredBucket<'a, Q, B>
    where Q: FirstInFirstOutQueue<B>,
          B: FirstInFirstOutBucket,
{
    fn enqueue(&mut self, item: Self::Item) {
        self.add().enqueue(item)
    }

    fn dequeue(&mut self) -> Option<Self::Item> {
        self.remove()?.dequeue()
    }
}
