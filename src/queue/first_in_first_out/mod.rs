use super::*;

pub trait FirstInFirstOutQueue<B: FirstInFirstOutBucket>: Queue<B> {
    fn enqueue(&mut self, item: B::Item, priority: usize) {
        self.get_or_insert_bucket_mut(priority).enqueue(item);
    }

    fn dequeue(&mut self, priority: usize) -> Option<B::Item> {
        self.get_bucket_mut(priority)?.dequeue()
    }
}

// -------------------------------------------------------------------------------
// Implement FirstInFirstOutQueue for BucketQueues that use FirstInFirstOutBucket:
// -------------------------------------------------------------------------------

impl<B: FirstInFirstOutBucket> FirstInFirstOutQueue<B> for BucketQueue<B> { }
