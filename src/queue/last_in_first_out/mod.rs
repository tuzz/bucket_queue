use super::*;

pub trait LastInFirstOutQueue<B: LastInFirstOutBucket>: Queue<B> {
    fn push(&mut self, item: B::Item, priority: usize) {
        self.bucket_for_adding(priority).push(item);
    }

    fn pop(&mut self, priority: usize) -> Option<B::Item> {
        self.bucket_for_removing(priority)?.pop()
    }

    fn pop_min(&mut self) -> Option<B::Item> {
        self.pop(self.min_priority()?)
    }

    fn pop_max(&mut self) -> Option<B::Item> {
        self.pop(self.max_priority()?)
    }
}

// -----------------------------------------------------------------------------
// Implement LastInFirstOutQueue for BucketQueues that use LastInFirstOutBucket:
// -----------------------------------------------------------------------------

impl<B: LastInFirstOutBucket> LastInFirstOutQueue<B> for BucketQueue<B> { }


// ---------------------------------------------------------------------
// Implement LastInFirstOutQueue for DeferredBucket to support deferral:
// ---------------------------------------------------------------------

impl<'a, Q, B, C> LastInFirstOutQueue<C> for DeferredBucket<'a, Q, B>
    where Q: Queue<B>,
          B: Bucket + Queue<C>,
          C: LastInFirstOutBucket,
{ }
