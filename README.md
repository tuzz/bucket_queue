## BucketQueue

[![Build Status](https://travis-ci.org/tuzz/bucket_queue.svg?branch=master)](https://travis-ci.org/tuzz/bucket_queue)
[![Latest version](https://img.shields.io/crates/v/bucket_queue.svg)](https://crates.io/crates/bucket_queue)
[![Rust Version](https://img.shields.io/badge/rust-2018%20edition-yellow.svg)](https://rust-lang-nursery.github.io/edition-guide/editions/index.html)
[![License](https://img.shields.io/github/license/mashape/apistatus.svg)](https://github.com/tuzz/bucket_queue/blob/master/LICENSE)

A priority queue that efficiently stores and retrieves items whose priorities
are small integers. Items are stored in 'buckets' which are other data structres
such as
[`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) or
[`VecDeque`](https://doc.rust-lang.org/std/collections/struct.VecDeque.html).
BucketQueue is designed to work with a variety of queueing semantics such as
[First-In-First-Out](https://en.wikipedia.org/wiki/FIFO_(computing_and_electronics)),
[Last-In-First-Out](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)) and
[Double-Ended](https://en.wikipedia.org/wiki/Double-ended_queue), but you can
extend it with your own.

This implementation is loosely based on the
[description from Wikipedia](https://en.wikipedia.org/wiki/Bucket_queue).

## Basic Usage

```rust
extern crate bucket_queue;

use bucket_queue::*;
use std::collections::VecDeque;

fn main() {
    // Initialize a queue with buckets that are VecDeque:
    let mut queue = BucketQueue::<VecDeque<&str>>::new();

    // Enqueue some items with associated priorities:
    queue.enqueue("refactor", 1);
    queue.enqueue("fix tests", 0);
    queue.enqueue("drink coffee", 1);
    queue.enqueue("documentation", 1);
    queue.enqueue("pull request", 2);

    // Dequeue items, ordered by minimum priority:
    assert_eq!(queue.dequeue_min(), Some("fix tests"));
    assert_eq!(queue.dequeue_min(), Some("refactor"));
    assert_eq!(queue.dequeue_min(), Some("drink coffee"));
    assert_eq!(queue.dequeue_min(), Some("documentation"));
    assert_eq!(queue.dequeue_min(), Some("pull request"));
    assert_eq!(queue.dequeue_min(), None);
}
```

**Things to note:**
- You need to `use bucket_queue::*` to pull in the required traits
- You can `dequeue_max` instead, if your priorities are reversed
- This example uses First-In-First-Out (FIFO) queueing semantics

## Last-In-First-Out

```rust
extern crate bucket_queue;

use bucket_queue::*;

fn main() {
    // Initialize a queue with buckets that are Vec:
    let mut queue = BucketQueue::<Vec<&str>>::new();

    // Push some items with associated priorities:
    queue.push("refactor", 1);
    queue.push("fix tests", 0);
    queue.push("drink coffee", 1);
    queue.push("documentation", 1);
    queue.push("pull request", 2);

    // Pop items, ordered by minimum priority:
    assert_eq!(queue.pop_min(), Some("fix tests"));
    assert_eq!(queue.pop_min(), Some("documentation")); //      ^
    assert_eq!(queue.pop_min(), Some("drink coffee"));  //      | reversed
    assert_eq!(queue.pop_min(), Some("refactor"));      //      v
    assert_eq!(queue.pop_min(), Some("pull request"));
    assert_eq!(queue.pop_min(), None);
}
```

**Things to note:**
- A `Vec` provides Last-In-First-Out (LIFO) queueing semantics
- We use `push` and `pop_min` instead of `enqueue` and `dequeue_min`
- The queueing semantics only affects the order of retrieval for items with
  equal priority

## Double-Ended

```rust
extern crate bucket_queue;

use bucket_queue::*;
use std::collections::VecDeque;

fn main() {
    // Initialize a queue with buckets that are VecDeque:
    let mut queue = BucketQueue::<VecDeque<&str>>::new();

    // Push some items with associated priorities:
    queue.push_back("refactor", 1);
    queue.push_back("fix tests", 0);
    queue.push_front("drink coffee", 1);  // <-- pushed to the front
    queue.push_back("documentation", 1);
    queue.push_back("pull request", 2);

    // Pop items, ordered by minimum priority:
    assert_eq!(queue.pop_front_min(), Some("fix tests"));
    assert_eq!(queue.pop_front_min(), Some("drink coffee"));
    assert_eq!(queue.pop_back_min(), Some("documentation"));   // <-- popped from the back
    assert_eq!(queue.pop_front_min(), Some("refactor"));
    assert_eq!(queue.pop_front_min(), Some("pull request"));
    assert_eq!(queue.pop_front_min(), None);
}
```

**Things to note:**
- A `VecDeque` (also) provides Double-Ended queueing semantics
- We can `push` and `pop` from both the front and the back

   Cargo.toml            |104     // Pop items, ord- Again, this priorities are still respected, this only affects ordering of
  items in buckets

## Utility Functions

```rust
extern crate bucket_queue;

use bucket_queue::*;
use std::collections::VecDeque;

fn main() {
    // Initialize a queue with buckets that are VecDeque:
    let mut queue = BucketQueue::<VecDeque<&str>>::new();

    // Enqueue some items with associated priorities:
    queue.enqueue("refactor", 1);
    queue.enqueue("fix tests", 0);
    queue.enqueue("drink coffee", 1);
    queue.enqueue("documentation", 1);
    queue.enqueue("pull request", 2);

    // Dequeue an item for a specific priority:
    assert_eq!(queue.dequeue(1), Some("refactor"));

    // Call some utility functions:
    assert_eq!(queue.len(), 4);
    assert_eq!(queue.is_empty(), false);
    assert_eq!(queue.min_priority(), Some(0));
    assert_eq!(queue.max_priority(), Some(2));

    // Remove all items from bucket 1:
    queue.prune(1);
    assert_eq!(queue.len(), 2);

    // Clear all items from the queue:
    queue.clear();

    assert_eq!(queue.len(), 0);
    assert_eq!(queue.is_empty(), true);
    assert_eq!(queue.min_priority(), None);
    assert_eq!(queue.max_priority(), None);
}
```

**Things to note:**
- You can `pop` / `pop_front` and `pop_back` an item for a specific priority, too
- BucketQueue does not implement
  [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
  because there are too many different ways to retrieve items

## Nested Queues

```rust
extern crate bucket_queue;

use bucket_queue::*;
use std::collections::VecDeque;

fn main() {
    // Initialize a queue with buckets that are themselves BucketQueue:
    let mut queue = BucketQueue::<BucketQueue<VecDeque<&str>>>::new();

    // Enqueue some items with two-dimensional priorities:
    queue.bucket(1).enqueue("refactor", 1);
    queue.bucket(0).enqueue("fix tests", 0);
    queue.bucket(1).enqueue("drink coffee", 0);
    queue.bucket(1).enqueue("documentation", 2);
    queue.bucket(2).enqueue("pull request", 0);

    // Dequeue items, ordered by minimum priority:
    assert_eq!(queue.min_bucket().dequeue_min(), Some("fix tests"));
    assert_eq!(queue.min_bucket().dequeue_min(), Some("drink coffee"));
    assert_eq!(queue.min_bucket().dequeue_min(), Some("refactor"));
    assert_eq!(queue.min_bucket().dequeue_min(), Some("documentation"));
    assert_eq!(queue.min_bucket().dequeue_min(), Some("pull request"));
    assert_eq!(queue.min_bucket().dequeue_min(), None);
}
```

**Things to note:**
- BucketQueue can be arbitrarily nested to any number of levels
- `min_bucket` and `max_bucket` find the bucket with minimum or maximum priority
- These are equivalent:

```rust
queue.bucket(1).enqueue("documentation", 2);
queue.bucket(1).bucket(2).enqueue("documentation");
```

- So are these:

```rust
queue.prune(0);
queue.bucket(0).clear();
```

## Tests

All tests for the crate are
[here.](https://github.com/tuzz/bucket_queue/blob/master/tests/bucket_queue.rs)
This can also be used as a reference.

## Benchmarks

```
test benchmark_100_items_into_4_buckets                  ... bench:       1,272 ns/iter (+/- 28)
test benchmark_1_000_items_into_8_buckets                ... bench:      12,103 ns/iter (+/- 1,157)
test benchmark_10_000_items_into_16_buckets              ... bench:     121,042 ns/iter (+/- 3,095)
test benchmark_100_000_items_into_32_buckets             ... bench:   1,214,780 ns/iter (+/- 24,987)
test benchmark_1_000_000_items_into_64_buckets           ... bench:  14,487,399 ns/iter (+/- 881,656)

test benchmark_100_items_into_4x4_nested_buckets         ... bench:       3,742 ns/iter (+/- 170)
test benchmark_1_000_items_into_8x8_nested_buckets       ... bench:      38,916 ns/iter (+/- 3,270)
test benchmark_10_000_items_into_16x16_nested_buckets    ... bench:     353,102 ns/iter (+/- 11,718)
test benchmark_100_000_items_into_32x32_nested_buckets   ... bench:   3,842,643 ns/iter (+/- 71,892)
test benchmark_1_000_000_items_into_64x64_nested_buckets ... bench:  47,129,660 ns/iter (+/- 726,701)
```

**Things to note:**
- These benchmarks were run on an
[Intel Core i5-4430 CPU](https://ark.intel.com/products/75036/Intel-Core-i5-4430-Processor-6M-Cache-up-to-3-20-GHz-)
- The slowest example (one million items into 64x64 nested buckets) took 47
milliseconds
- These benchmarks can be run with `cargo bench`

## Adding a new queueing semantic

In this example, we'll introduce a `BiggestFirstQueue`. This will retrieve items
from buckets, biggest to smallest. BucketQueue's priorities will still be
respected, but when items have equal priority, the biggest will be returned
first.

There's quite a lot boilerplate required (sorry). This is mostly a result of
trying to make things flexible. I've broken it down into steps.

### Step 1: Define a new type of `Bucket`:

```rust
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
```

**Things to note:**
- This example uses a `BinaryHeap` to store items
- It needs to be wrapped in a struct due to Rust's
[orphan rules](https://doc.rust-lang.org/error-index.html#E0210)
- The `Ord` constraint is imposed by `BinaryHeap`, not `BucketQueue`
- Most of this is boilerplate that proxies calls through to `BinaryHeap`

### Step 2: Define how the bucket works with items:

```rust
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
```

**Things to note:**
- `BiggestFirstBucket` has a
  [supertrait](https://doc.rust-lang.org/1.25.0/reference/items/traits.html#supertraits)
  of `Bucket`
- Items are added to the bucket with `insert` and retrieved with `biggest`
- This trait is implemented for `Heap`, which calls `push` and `pop` on the `BinaryHeap`

### Step 3: Define a new type of `Queue` for our `Bucket`:

```rust
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
```

**Things to note:**
- `bucket_for_adding` and `bucket_for_removing` are internal functions that keep
  BucketQueue's index up to date
- `biggest` retrieves from the minimum priority bucket, but we could add
  `biggest_min` and `biggest_max` if we wanted
- The last line adds support for this queueing semantic to `BucketQueue`

### Finally, we can use it:

```rust
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
}
```

**Things to note:**
- `BucketQueue` uses our custom `Heap` type
- The queueing semantics are inferred from the type of `Bucket` used
- `donkey` has a priority of `1` so it appears at the end
- This example can be seen in full
  [here](https://github.com/tuzz/bucket_queue/blob/master/src/bin/custom_queue.rs)
  and can be run with `cargo run`

### (Optional) Step 4: Add support for nesting:

To make your new type of queue work when `BucketQueue` is nested, you'll need an
extra bit of boilerplate:

```rust
impl<'a, Q, B, C> BiggestFirstQueue<C> for DeferredBucket<'a, Q, B>
    where Q: Queue<B>, B: Bucket + Queue<C>, C: BiggestFirstBucket { }

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
```

**Things to note:**
- This is all boilerplate and calls through to functions already defined
- `adding` and `removing` are internal functions that keep BucketQueue's index
  up to date

### Final example with nesting:

```rust
fn main() {
    // Initialize a queue with buckets that are themselves BucketQueue:
    let mut queue = BucketQueue::<BucketQueue<Heap<&str>>>::new();

    // Insert some items into nested buckets:
    queue.bucket(0).insert("aardvark", 0);
    queue.bucket(0).insert("barn owl", 0);
    queue.bucket(1).bucket(1).insert("crocodile");
    queue.bucket(1).bucket(0).insert("donkey");

    // Retrieve the items from nested buckets:
    assert_eq!(queue.min_bucket().biggest(), Some("barn owl"));
    assert_eq!(queue.min_bucket().biggest(), Some("aardvark"));
    assert_eq!(queue.min_bucket().biggest(), Some("donkey"));
    assert_eq!(queue.min_bucket().biggest(), Some("crocodile"));
    assert_eq!(queue.min_bucket().biggest(), None);
}
```

**Things to note:**
- This example uses the two equivalent ways to insert items (see above: search
  for 'equivalent')

## Implementation Notes

As you've probably seen above, a lot of traits are used to make BucketQueue more
flexible. This adds boilerplate, but it means custom queueing semantics can be
added, or existing semantics can be built on different data structures.

There's also an `Index` trait, which currently has a single implementation
called `SimpleIndex`. This implements the lower- and upper-bounds optimisation
[described on Wikipedia](https://en.wikipedia.org/wiki/Bucket_queue#Optimizations).

In theory, it would be possible to extend BucketQueue with better indexing
strategies, perhaps using a `BinaryHeap` or `HashMap`. To use a custom `Index`,
you'd initialize `BucketQueue` like so:

```rust
let queue = BucketQueue::<SomeBucket<&str>,MyCustomIndex>::new();
```

For example:

```rust
let queue = BucketQueue::<Vec<&str>,MyIndexThatUsesAHeap>::new();
```

I considered exploring better indexing strategies, but decided against it to
keep the scope of this project under control.

Finally, one last thing to point out is that, although these are functionally
equivalent:

```rust
queue.bucket(0).bucket(1).enqueue("something");
queue.bucket(0).enqueue("something", 1);
```

There's a small performance overhead in the former. This is because it
constructs a `DeferredBucket` for every call to `bucket`. In the most
time-consuming benchmark, this overhead slows things down by about 7%. For
time-critical use cases, you can do this instead:

```rust
queue.bucket_for_adding(0).enqueue("something", 1)
```

This bypasses the `DeferredBucket`, but there's more danger the `Index` becomes
out-of-sync, if you accidentally do this:

```rust
queue.bucket_for_adding(0).dequeue_min(); // THIS IS WRONG
```

The problem is that you've informed the queue you'll be adding an item, then
removed one, putting the `Index` into an inconsistent state. I thought about
whether the same flexibility could be granted, in a consistent way, without the
overhead, but didn't manage to find a way to do this. Perhaps someone with more
experience of Rust's generics and traits can.

## Contribution

All contributions are welcome. At time of writing I've been using Rust for about
six months so I'm sure there's plenty of area for improvement. Please open an
issue or create a pull request. Ping
[me on Twitter](https://twitter.com/chrispatuzzo) if I'm unresponsive.
