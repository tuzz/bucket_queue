use super::*;

pub trait DoubleEndedQueue<B: DoubleEndedBucket>: Queue<B> {
    fn push_back(&mut self, item: B::Item, priority: usize) {
        self.bucket_for_adding(priority).push_back(item);
    }

    fn push_front(&mut self, item: B::Item, priority: usize) {
        self.bucket_for_adding(priority).push_front(item);
    }

    fn pop_back(&mut self, priority: usize) -> Option<B::Item> {
        self.bucket_for_removing(priority)?.pop_back()
    }

    fn pop_front(&mut self, priority: usize) -> Option<B::Item> {
        self.bucket_for_removing(priority)?.pop_front()
    }

    fn pop_back_min(&mut self) -> Option<B::Item> {
        self.pop_back(self.min_priority()?)
    }

    fn pop_front_min(&mut self) -> Option<B::Item> {
        self.pop_front(self.min_priority()?)
    }

    fn pop_back_max(&mut self) -> Option<B::Item> {
        self.pop_back(self.max_priority()?)
    }

    fn pop_front_max(&mut self) -> Option<B::Item> {
        self.pop_front(self.max_priority()?)
    }
}

// -----------------------------------------------------------------------
// Implement DoubleEndedQueue for BucketQueues that use DoubleEndedBucket:
// -----------------------------------------------------------------------

impl<B: DoubleEndedBucket> DoubleEndedQueue<B> for BucketQueue<B> { }


// ------------------------------------------------------------------
// Implement DoubleEndedQueue for DeferredBucket to support deferral:
// ------------------------------------------------------------------

impl<'a, Q, B, C> DoubleEndedQueue<C> for DeferredBucket<'a, Q, B>
    where Q: Queue<B>,
          B: Bucket + Queue<C>,
          C: DoubleEndedBucket,
{ }
