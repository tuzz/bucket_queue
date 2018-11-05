use super::*;

pub trait LastInFirstOutQueue<B: Bucket> {
    fn push(&mut self, item: B::Item, priority: usize);

    fn pop(&mut self, priority: usize) -> Option<B::Item>;
}

// -----------------------------------------------------------------------------
// Implement LastInFirstOutQueue for BucketQueues that use LastInFirstOutBucket:
// -----------------------------------------------------------------------------

impl<B> LastInFirstOutQueue<B> for BucketQueue<B>
    where B: LastInFirstOutBucket
{
    fn push(&mut self, item: B::Item, priority: usize) {
        self.get_or_insert_bucket_mut(priority).push(item);
    }

    fn pop(&mut self, priority: usize) -> Option<B::Item> {
        self.get_bucket_mut(priority)?.pop()
    }
}
