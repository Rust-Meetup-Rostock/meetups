use super::utils::*;
use std::ops::Range;

#[derive(Clone)]
pub struct ProcessingData {
    pub numbers: Vec<i32>,
    pub range: Range<i32>,
    pub strings: Vec<String>,
}

impl Default for ProcessingData {
    fn default() -> Self {
        Self {
            numbers: get_random_int_vec(100_000_000),
            range: 0..10,
            strings: get_random_str_vec(10_000),
        }
    }
}
