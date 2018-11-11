use super::*;

pub trait DoubleEndedQueue<B: DoubleEndedBucket>: Queue<B> {
    fn push_back(&mut self, item: B::Item, priority: usize) {
        self.get_or_insert_bucket_mut(priority).push_back(item);
    }

    fn push_front(&mut self, item: B::Item, priority: usize) {
        self.get_or_insert_bucket_mut(priority).push_front(item);
    }

    fn pop_back(&mut self, priority: usize) -> Option<B::Item> {
        self.get_bucket_mut(priority)?.pop_back()
    }

    fn pop_front(&mut self, priority: usize) -> Option<B::Item> {
        self.get_bucket_mut(priority)?.pop_front()
    }
}

// -----------------------------------------------------------------------
// Implement DoubleEndedQueue for BucketQueues that use DoubleEndedBucket:
// -----------------------------------------------------------------------

impl<B: DoubleEndedBucket> DoubleEndedQueue<B> for BucketQueue<B> { }
