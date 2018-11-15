extern crate bucket_queue;

use bucket_queue::*;

// ------------------------------------
// Step 1: Define a new type of Bucket:
// ------------------------------------

use std::collections::BinaryHeap;

struct Heap<T> {
    binary_heap: BinaryHeap<T>
}

impl<T: Ord> Bucket for Heap<T> {
    type Item = T;

    fn new_bucket() -> Self {
        Heap { binary_heap: BinaryHeap::new() }
    }

    fn len_bucket(&self) -> usize {
        self.binary_heap.len()
    }

    fn is_empty_bucket(&self) -> bool {
        self.binary_heap.is_empty()
    }

    fn clear(&mut self) {
        self.binary_heap.clear()
    }
}


// -----------------------------------------------
// Step 2: Define how the bucket works with items:
// -----------------------------------------------

trait BiggestFirstBucket: Bucket {
    fn insert(&mut self, item: Self::Item);

    fn biggest(&mut self) -> Option<Self::Item>;
}

impl<T: Ord> BiggestFirstBucket for Heap<T> {
    fn insert(&mut self, item: Self::Item) {
        self.binary_heap.push(item)
    }

    fn biggest(&mut self) -> Option<Self::Item> {
        self.binary_heap.pop()
    }
}


// --------------------------------------------------
// Step 3: Define a new type of Queue for our Bucket:
// --------------------------------------------------

trait BiggestFirstQueue<B: BiggestFirstBucket>: Queue<B> {
    fn insert(&mut self, item: B::Item, priority: usize) {
        self.bucket_for_adding(priority).insert(item);
    }

    fn biggest(&mut self) -> Option<B::Item> {
        let priority = self.min_priority()?;
        self.bucket_for_removing(priority)?.biggest()
    }
}

impl<B: BiggestFirstBucket> BiggestFirstQueue<B> for BucketQueue<B> { }


// -----------------------
// Finally, we can use it:
// -----------------------

fn main() {
    // Initialize a queue with buckets that are Heap:
    let mut queue = BucketQueue::<Heap<&str>>::new();

    // Insert some items with associated priorities:
    queue.insert("aardvark", 0);
    queue.insert("barn owl", 0);
    queue.insert("crocodile", 0);
    queue.insert("donkey", 1);

    // Retrieve the items reverse alphabetically, ordered by minimum priority:
    assert_eq!(queue.biggest(), Some("crocodile"));
    assert_eq!(queue.biggest(), Some("barn owl"));
    assert_eq!(queue.biggest(), Some("aardvark"));
    assert_eq!(queue.biggest(), Some("donkey"));
    assert_eq!(queue.biggest(), None);

    main_with_nesting();
}


// -------------------------------------------
// (Optional) Step 4: Add support for nesting:
// -------------------------------------------

impl<'a, Q, B> BiggestFirstBucket for DeferredBucket<'a, Q, B>
    where Q: BiggestFirstQueue<B>, B: BiggestFirstBucket
{
    fn insert(&mut self, item: Self::Item) {
        self.adding().insert(item);
    }

    fn biggest(&mut self) -> Option<Self::Item> {
        self.removing()?.biggest()
    }
}

impl<'a, Q, B, C> BiggestFirstQueue<C> for DeferredBucket<'a, Q, B>
    where Q: Queue<B>, B: Bucket + Queue<C>, C: BiggestFirstBucket { }


// ---------------------------
// Final example with nesting:
// ---------------------------

fn main_with_nesting() {
    // Initialize a queue with buckets that are themselves BucketQueue:
    let mut queue = BucketQueue::<BucketQueue<Heap<&str>>>::new();

    // Insert some items into nested buckets:
    queue.bucket(0).insert("aardvark", 0);
    queue.bucket(0).insert("barn owl", 0);          // <-- These are equivalent
    queue.bucket(1).bucket(1).insert("crocodile");  //     ways to insert items
    queue.bucket(1).bucket(0).insert("donkey");

    // Retrieve the items from nested buckets:
    assert_eq!(queue.min_bucket().biggest(), Some("barn owl"));
    assert_eq!(queue.min_bucket().biggest(), Some("aardvark"));
    assert_eq!(queue.min_bucket().biggest(), Some("donkey"));
    assert_eq!(queue.min_bucket().biggest(), Some("crocodile"));
    assert_eq!(queue.min_bucket().biggest(), None);
}
