use super::*;

pub trait LastInFirstOutBucket: Bucket {
    fn push(&mut self, item: Self::Item);

    fn pop(&mut self) -> Option<Self::Item>;
}

// ---------------------------------------------------------------------
// Provide a canonical implementation of LastInFirstOutBucket using Vec:
// ---------------------------------------------------------------------

impl<T> LastInFirstOutBucket for Vec<T> {
    fn push(&mut self, item: Self::Item) {
        self.push(item)
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }
}
