pub mod double_ended;
pub mod first_in_first_out;
pub mod last_in_first_out;

pub trait Bucket {
    type Item;

    fn new() -> Self;
}

// -----------------------------------------------------------------
// Provide canonical implementations of Bucket for VecDeque and Vec:
// -----------------------------------------------------------------

use std::collections::VecDeque;

impl<T> Bucket for VecDeque<T> {
    type Item = T;

    fn new() -> Self {
        VecDeque::new()
    }
}

impl<T> Bucket for Vec<T> {
    type Item = T;

    fn new() -> Self {
        Vec::new()
    }
}
