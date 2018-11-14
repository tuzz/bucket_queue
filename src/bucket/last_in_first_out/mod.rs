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


// ----------------------------------------------------------------------
// Implement LastInFirstOutBucket for DeferredBucket to support deferral:
// ----------------------------------------------------------------------

impl<'a, Q, B> LastInFirstOutBucket for DeferredBucket<'a, Q, B>
    where Q: LastInFirstOutQueue<B>,
          B: LastInFirstOutBucket,
{
    fn push(&mut self, item: Self::Item) {
        self.adding().push(item)
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.removing()?.pop()
    }
}
