use super::*;

use std::cmp;

pub struct SimpleIndex {
    len: usize,
    min: Option<usize>,
    max: Option<usize>,
}

impl Index for SimpleIndex {
    fn new() -> Self {
        Self { len: 0, min: None, max: None }
    }

    fn add<B: Bucket>(&mut self, priority: usize, _: &Vec<Option<B>>) {
        self.len += 1;

        self.min = Self::compare(cmp::min, self.min, priority);
        self.max = Self::compare(cmp::max, self.max, priority);
    }

    fn remove<B: Bucket>(&mut self, priority: usize, buckets: &Vec<Option<B>>) {
        self.len = self.len.saturating_sub(1);

        if Self::size_of_bucket(priority, buckets) > 1 {
            return;
        }

        if let (Some(min), Some(max)) = (self.min, self.max) {
            if priority == min {
                self.min = Self::find_next_priority((min + 1)..=max, buckets);
            }

            if priority == max {
                self.max = Self::find_next_priority((min..max).rev(), buckets);
            }
        }
    }

    fn min(&self) -> Option<usize> {
        self.min
    }

    fn max(&self) -> Option<usize> {
        self.max
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl SimpleIndex {
    fn compare<F>(func: F, left: Option<usize>, right: usize) -> Option<usize>
        where F: Fn(usize, usize) -> usize
    {
        Some(match left {
            Some(value) => func(value, right),
            None => right,
        })
    }

    fn size_of_bucket<B: Bucket>(priority: usize, buckets: &Vec<Option<B>>) -> usize {
        let bucket = match buckets.get(priority) {
            None => return 0,
            Some(option) => match option {
                None => return 0,
                Some(bucket) => bucket,
            }
        };

        bucket.len_bucket()
    }

    fn find_next_priority<I, B>(iter: I, buckets: &Vec<Option<B>>) -> Option<usize>
        where I: Iterator<Item=usize>, B: Bucket
    {
        for i in iter {
            if let Some(bucket) = &buckets[i] {
                if !bucket.is_empty_bucket() {
                    return Some(i);
                }
            }
        }

        None
    }
}
