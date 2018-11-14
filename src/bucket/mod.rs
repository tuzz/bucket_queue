pub mod double_ended;
pub mod first_in_first_out;
pub mod last_in_first_out;

use super::*;

pub trait Bucket {
    type Item;

    fn new_bucket() -> Self;

    fn len_bucket(&self) -> usize;

    fn is_empty_bucket(&self) -> bool;

    fn clear(&mut self);
}

// -----------------------------------------------------------------
// Provide canonical implementations of Bucket for VecDeque and Vec:
// -----------------------------------------------------------------

use std::collections::VecDeque;

impl<T> Bucket for VecDeque<T> {
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
        self.clear()
    }
}

impl<T> Bucket for Vec<T> {
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
        self.clear()
    }
}
