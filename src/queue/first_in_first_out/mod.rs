use super::*;

pub trait FirstInFirstOutQueue<B: FirstInFirstOutBucket>: Queue<B> {
    fn enqueue(&mut self, item: B::Item, priority: usize) {
        self.bucket_for_adding(priority).enqueue(item);
    }

    fn dequeue(&mut self, priority: usize) -> Option<B::Item> {
        self.bucket_for_removing(priority)?.dequeue()
    }

    fn dequeue_min(&mut self) -> Option<B::Item> {
        self.dequeue(self.min_priority()?)
    }

    fn dequeue_max(&mut self) -> Option<B::Item> {
        self.dequeue(self.max_priority()?)
    }
}

// -------------------------------------------------------------------------------
// Implement FirstInFirstOutQueue for BucketQueues that use FirstInFirstOutBucket:
// -------------------------------------------------------------------------------

impl<B: FirstInFirstOutBucket> FirstInFirstOutQueue<B> for BucketQueue<B> { }


// ----------------------------------------------------------------------
// Implement FirstInFirstOutQueue for DeferredBucket to support deferral:
// ----------------------------------------------------------------------

impl<'a, Q, B, C> FirstInFirstOutQueue<C> for DeferredBucket<'a, Q, B>
    where Q: Queue<B>,
          B: Bucket + Queue<C>,
          C: FirstInFirstOutBucket,
{ }
