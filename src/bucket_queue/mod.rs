use super::*;

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

    fn grow(&mut self, priority: usize) {
        for _ in self.buckets.len()..=priority {
            self.buckets.push(None);
        }
    }

    fn get_bucket_unchecked(&mut self, priority: usize) -> &mut Option<B> {
        unsafe {
            self.buckets.get_unchecked_mut(priority)
        }
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

        self.grow(priority);

        self.get_bucket_unchecked(priority)
            .get_or_insert_with(|| B::new_bucket())
    }

    fn bucket_for_removing(&mut self, priority: usize) -> Option<&mut B> {
        self.index.remove(priority, &self.buckets);

        self.buckets.get_mut(priority)?.as_mut()
    }

    fn bucket_for_peeking(&self, priority: usize) -> Option<&B> {
        self.buckets.get(priority)?.as_ref()
    }

    fn bucket_for_pruning(&mut self, priority: usize) -> Option<&mut B> {
        // The index is not automatically updated as there is no way to tell how
        // many items will be removed. Instead, #items_pruned must be called.

        self.buckets.get_mut(priority)?.as_mut()
    }

    fn items_pruned(&mut self, number_of_items: usize, priority: usize) {
        self.index.removed_n(number_of_items, priority, &self.buckets);
    }

    fn len_queue(&self) -> usize {
        self.len()
    }

    fn is_empty_queue(&self) -> bool {
        self.is_empty()
    }

    fn prune(&mut self, priority: usize) {
        if let Some(Some(bucket)) = self.buckets.get_mut(priority) {
            let bucket_size = bucket.len_bucket();

            bucket.clear();

            self.items_pruned(bucket_size, priority);
        }
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
