use super::*;

pub struct BucketQueue<B: Bucket> {
    buckets: Vec<Option<B>>,
}

impl<B: Bucket> BucketQueue<B> {
    pub fn new() -> Self {
        Self { buckets: Vec::new() }
    }

    fn grow(&mut self, priority: usize) {
        for _ in self.buckets.len()..=priority {
            self.buckets.push(None);
        }
    }
}

impl<B: Bucket> Queue<B> for BucketQueue<B> {
    fn get_bucket_mut(&mut self, priority: usize) -> Option<&mut B> {
        self.buckets.get_mut(priority)?.as_mut()
    }

    fn get_or_insert_bucket_mut(&mut self, priority: usize) -> &mut B {
        self.grow(priority);

        let option = unsafe {
            self.buckets.get_unchecked_mut(priority)
        };

        option.get_or_insert_with(|| B::new())
    }
}

#[cfg(test)]
mod test;
