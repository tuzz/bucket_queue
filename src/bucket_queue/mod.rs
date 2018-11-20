use super::*;
use std::mem::replace;

pub struct BucketQueue<B: Bucket, I: Index = SimpleIndex> {
    buckets: Vec<Option<B>>,
    index: I,
}

impl<B: Bucket, I: Index> BucketQueue<B, I> {
    pub fn new() -> Self {
        Self { buckets: Vec::new(), index: I::new() }
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    fn grow(&mut self, priority: usize) -> &mut Option<B> {
        for _ in self.buckets.len()..=priority {
            self.buckets.push(None);
        }

        &mut self.buckets[priority]
    }
}

impl<B: Bucket> Queue<B> for BucketQueue<B> {
    fn new_queue() -> Self {
        Self::new()
    }

    fn min_priority(&self) -> Option<usize> {
        self.index.min()
    }

    fn max_priority(&self) -> Option<usize> {
        self.index.max()
    }

    fn bucket_for_adding(&mut self, priority: usize) -> &mut B {
        self.index.add(priority, &self.buckets);

        self.grow(priority).get_or_insert_with(|| B::new_bucket())
    }

    fn bucket_for_removing(&mut self, priority: usize) -> Option<&mut B> {
        self.index.remove(priority, &self.buckets);

        self.buckets.get_mut(priority)?.as_mut()
    }

    fn bucket_for_peeking(&self, priority: usize) -> Option<&B> {
        self.buckets.get(priority)?.as_ref()
    }

    fn bucket_for_replacing(&mut self, priority: usize) -> &mut Option<B> {
        // The index is not automatically updated as there is no way to tell how
        // many items will be replaced. Instead, #items_replaced must be called.

        self.grow(priority)
    }

    fn items_replaced(&mut self, priority: usize, old_size: usize, new_size: usize) {
        if new_size > old_size {
            self.index.added_n(new_size - old_size, priority, &self.buckets);
        } else if new_size < old_size {
            self.index.removed_n(old_size - new_size, priority, &self.buckets);
        }
    }

    fn len_queue(&self) -> usize {
        self.len()
    }

    fn is_empty_queue(&self) -> bool {
        self.is_empty()
    }

    fn replace(&mut self, priority: usize, replacement: Option<B>) -> Option<B> {
        let existing = self.grow(priority);

        let old_size = existing.as_ref().map_or(0, |b| b.len_bucket());
        let new_size = replacement.as_ref().map_or(0, |b| b.len_bucket());

        let replaced = replace(existing, replacement);
        self.items_replaced(priority, old_size, new_size);

        replaced
    }
}

impl<T, B: Bucket<Item=T>, I: Index> Bucket for BucketQueue<B, I> {
    type Item = T;

    fn new_bucket() -> Self {
        Self::new()
    }

    fn len_bucket(&self) -> usize {
        self.len()
    }

    fn is_empty_bucket(&self) -> bool {
        self.is_empty()
    }

    fn clear(&mut self) {
        self.buckets = Vec::new();
        self.index = I::new();
    }
}
