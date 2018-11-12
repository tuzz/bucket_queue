pub mod double_ended;
pub mod first_in_first_out;
pub mod last_in_first_out;

use super::*;

pub trait Queue<B: Bucket> {
    fn min_priority(&self) -> Option<usize>;
    fn max_priority(&self) -> Option<usize>;

    fn bucket_for_adding(&mut self, priority: usize) -> &mut B;
    fn bucket_for_removing(&mut self, priority: usize) -> Option<&mut B>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;
}
