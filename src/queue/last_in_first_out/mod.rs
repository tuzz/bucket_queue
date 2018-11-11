use super::*;

pub trait LastInFirstOutQueue<B: LastInFirstOutBucket>: Queue<B> {
    fn push(&mut self, item: B::Item, priority: usize) {
        self.get_or_insert_bucket_mut(priority).push(item);
    }

    fn pop(&mut self, priority: usize) -> Option<B::Item> {
        self.get_bucket_mut(priority)?.pop()
    }
}

// -----------------------------------------------------------------------------
// Implement LastInFirstOutQueue for BucketQueues that use LastInFirstOutBucket:
// -----------------------------------------------------------------------------

impl<B: LastInFirstOutBucket> LastInFirstOutQueue<B> for BucketQueue<B> { }
