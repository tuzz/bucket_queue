use super::*;

pub struct BucketQueue<B: Bucket, I: Index = SimpleIndex> {
    buckets: Vec<Option<B>>,
    index: I,
}

impl<B: Bucket, I: Index> BucketQueue<B, I> {
    pub fn new() -> Self {
        Self { buckets: Vec::new(), index: I::new() }
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
            .get_or_insert_with(|| B::new())
    }

    fn bucket_for_removing(&mut self, priority: usize) -> Option<&mut B> {
        self.index.remove(priority, &self.buckets);

        self.buckets.get_mut(priority)?.as_mut()
    }
}

#[cfg(test)]
mod test;
