pub mod bucket;

use super::*;

pub trait Deferred<B: Bucket>: Queue<B> + Sized {
    fn bucket(&mut self, priority: usize) -> DeferredBucket<Self, B> {
        DeferredBucket::new(self, priority)
    }

    fn min_bucket(&mut self) -> DeferredBucket<Self, B> {
        self.bucket(self.min_priority().unwrap_or(0))
    }

    fn max_bucket(&mut self) -> DeferredBucket<Self, B> {
        self.bucket(self.max_priority().unwrap_or(0))
    }
}

impl<B: Bucket> Deferred<B> for BucketQueue<B> { }

impl<'a, Q, B, C> Deferred<C> for DeferredBucket<'a, Q, B>
    where Q: Queue<B>,
          B: Bucket + Queue<C>,
          C: Bucket,
{ }
