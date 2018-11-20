pub mod simple;

use super::*;

pub trait Index {
    fn new() -> Self;

    fn add<B: Bucket>(&mut self, priority: usize, buckets: &Vec<Option<B>>);
    fn remove<B: Bucket>(&mut self, priority: usize, buckets: &Vec<Option<B>>);

    fn added_n<B: Bucket>(&mut self, n: usize, priority: usize, buckets: &Vec<Option<B>>);
    fn removed_n<B: Bucket>(&mut self, n: usize, priority: usize, buckets: &Vec<Option<B>>);

    fn min(&self) -> Option<usize>;
    fn max(&self) -> Option<usize>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}
