use super::*;

pub trait DoubleEndedQueue<B: Bucket> {
    fn push_back(&mut self, item: B::Item, priority: usize);
    fn push_front(&mut self, item: B::Item, priority: usize);

    fn pop_back(&mut self, priority: usize) -> Option<B::Item>;
    fn pop_front(&mut self, priority: usize) -> Option<B::Item>;
}

// -----------------------------------------------------------------------
// Implement DoubleEndedQueue for BucketQueues that use DoubleEndedBucket:
// -----------------------------------------------------------------------

impl<B> DoubleEndedQueue<B> for BucketQueue<B>
    where B: DoubleEndedBucket
{
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
