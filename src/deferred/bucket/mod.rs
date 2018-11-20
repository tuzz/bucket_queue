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

    pub fn len(&self) -> usize {
        self.peeking().map_or(0, |b| b.len_bucket())
    }

    pub fn is_empty(&self) -> bool {
        self.peeking().map_or(true, |b| b.is_empty_bucket())
    }

    pub fn adding(&mut self) -> &mut B {
        self.panic_if_consumed();
        self.queue.bucket_for_adding(self.priority)
    }

    pub fn removing(&mut self) -> Option<&mut B> {
        self.panic_if_consumed();
        self.queue.bucket_for_removing(self.priority)
    }

    pub fn peeking(&self) -> Option<&B> {
        self.queue.bucket_for_peeking(self.priority)
    }

    pub fn pruning(&mut self) -> Option<&mut B> {
        self.queue.bucket_for_pruning(self.priority)
    }

    // Updates queue's index to record how many items were pruned.
    pub fn pruned(&mut self, number_of_items: usize) {
        self.queue.items_pruned(number_of_items, self.priority);
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

    fn clear(&mut self) {
        self.queue.prune(self.priority);
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
        self.peeking()?.min_priority()
    }

    fn max_priority(&self) -> Option<usize> {
        self.peeking()?.max_priority()
    }

    fn bucket_for_adding(&mut self, priority: usize) -> &mut C {
        self.adding().bucket_for_adding(priority)
    }

    fn bucket_for_removing(&mut self, priority: usize) -> Option<&mut C> {
        self.removing()?.bucket_for_removing(priority)
    }

    fn bucket_for_peeking(&self, priority: usize) -> Option<&C> {
        self.peeking()?.bucket_for_peeking(priority)
    }

    fn bucket_for_pruning(&mut self, priority: usize) -> Option<&mut C> {
        self.pruning()?.bucket_for_pruning(priority)
    }

    fn items_pruned(&mut self, number_of_items: usize, priority: usize) {
        // Update the parent queue's index.
        self.pruned(number_of_items);

        // Update the current queue's index.
        self.pruning().map(|q| q.items_pruned(number_of_items, priority));
    }

    fn len_queue(&self) -> usize {
        self.peeking().map_or(0, |q| q.len_queue())
    }

    fn is_empty_queue(&self) -> bool {
        self.peeking().map_or(true, |q| q.is_empty_queue())
    }

    fn prune(&mut self, priority: usize) -> Option<C> {
        let queue = self.pruning()?;
        let bucket_size = queue.bucket_for_peeking(priority)?.len_bucket();
        let option = queue.prune(priority);

        self.pruned(bucket_size);

        option
    }
}
