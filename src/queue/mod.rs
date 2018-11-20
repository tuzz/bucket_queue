pub mod double_ended;
pub mod first_in_first_out;
pub mod last_in_first_out;

use super::*;

pub trait Queue<B: Bucket> {
    fn new_queue() -> Self;

    fn min_priority(&self) -> Option<usize>;
    fn max_priority(&self) -> Option<usize>;

    fn bucket_for_adding(&mut self, priority: usize) -> &mut B;
    fn bucket_for_removing(&mut self, priority: usize) -> Option<&mut B>;
    fn bucket_for_peeking(&self, priority: usize) -> Option<&B>;
    fn bucket_for_replacing(&mut self, priority: usize) -> &mut Option<B>;

    fn items_replaced(&mut self, priority: usize, old_size: usize, new_size: usize);

    fn len_queue(&self) -> usize;
    fn is_empty_queue(&self) -> bool;

    fn replace(&mut self, priority: usize, replacement: Option<B>) -> Option<B>;
}
