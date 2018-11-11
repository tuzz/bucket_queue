pub mod double_ended;
pub mod first_in_first_out;
pub mod last_in_first_out;

use super::*;

pub trait Queue<B: Bucket> {
    fn get_bucket_mut(&mut self, priority: usize) -> Option<&mut B>;

    fn get_or_insert_bucket_mut(&mut self, priority: usize) -> &mut B;
}
