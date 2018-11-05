use super::*;

pub trait FirstInFirstOutQueue<B: Bucket> {
    fn enqueue(&mut self, item: B::Item, priority: usize);

    fn dequeue(&mut self, priority: usize) -> Option<B::Item>;
}

// -------------------------------------------------------------------------------
// Implement FirstInFirstOutQueue for BucketQueues that use FirstInFirstOutBucket:
// -------------------------------------------------------------------------------

impl<B> FirstInFirstOutQueue<B> for BucketQueue<B>
    where B: FirstInFirstOutBucket
{
    fn enqueue(&mut self, item: B::Item, priority: usize) {
        self.get_or_insert_bucket_mut(priority).enqueue(item);
    }

    fn dequeue(&mut self, priority: usize) -> Option<B::Item> {
        self.get_bucket_mut(priority)?.dequeue()
    }
}
