use super::*;

use std::marker::PhantomData;

pub struct DeferredBucket<'a, Q, B>
    where Q: Queue<B>,
          B: Bucket,
{
    queue: &'a mut Q,
    priority: usize,
    consumed: bool,
    phantom: PhantomData<B>,
}

impl<'a, Q, B> DeferredBucket<'a, Q, B>
    where Q: Queue<B>,
          B: Bucket,
{
    pub fn new(queue: &'a mut Q, priority: usize) -> Self {
        Self { queue, priority, consumed: false, phantom: PhantomData }
    }

    pub fn add(&mut self) -> &mut B {
        self.panic_if_consumed();
        self.queue.bucket_for_adding(self.priority)
    }

    pub fn remove(&mut self) -> Option<&mut B> {
        self.panic_if_consumed();
        self.queue.bucket_for_removing(self.priority)
    }

    pub fn peek(&self) -> Option<&B> {
        self.queue.bucket_for_peeking(self.priority)
    }

    fn panic_if_consumed(&mut self) {
        if self.consumed {
            panic!("You may only add or remove a single item from the bucket.")
        }

        self.consumed = true
    }
}

impl<'a, Q, B> Bucket for DeferredBucket<'a, Q, B>
    where Q: Queue<B>,
          B: Bucket,
{
    type Item = B::Item;

    fn new_bucket() -> Self {
        panic!("DeferredBucket should not be initialized this way.");
    }

    fn len_bucket(&self) -> usize {
        self.queue.len_queue()
    }

    fn is_empty_bucket(&self) -> bool {
        self.queue.is_empty_queue()
    }
}

// ----------------------------------------------------------------------
// Implement Queue for DeferredBucket if the Bucket type is also a Queue:
// ----------------------------------------------------------------------

impl<'a, Q, B, C> Queue<C> for DeferredBucket<'a, Q, B>
    where Q: Queue<B>,
          B: Bucket + Queue<C>,
          C: Bucket,
{
    fn new_queue() -> Self {
        panic!("DeferredBucket should not be initialized this way.");
    }

    fn min_priority(&self) -> Option<usize> {
        self.peek()?.min_priority()
    }

    fn max_priority(&self) -> Option<usize> {
        self.peek()?.max_priority()
    }

    fn bucket_for_adding(&mut self, priority: usize) -> &mut C {
        self.add().bucket_for_adding(priority)
    }

    fn bucket_for_removing(&mut self, priority: usize) -> Option<&mut C> {
        self.remove()?.bucket_for_removing(priority)
    }

    fn bucket_for_peeking(&self, priority: usize) -> Option<&C> {
        self.peek()?.bucket_for_peeking(priority)
    }

    fn len_queue(&self) -> usize {
        self.peek().map_or(0, |q| q.len_queue())
    }

    fn is_empty_queue(&self) -> bool {
        self.peek().map_or(true, |q| q.is_empty_queue())
    }
}
