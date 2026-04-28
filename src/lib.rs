#![no_std]

extern crate alloc;

mod map;
mod set;

pub use map::{ConstantFlatMap, FlatMap, FlatMapEntry};
pub use set::{ConstantFlatSet, FlatSet};
