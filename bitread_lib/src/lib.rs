#![allow(unused)]
pub use bitvec::prelude::*;

// Define the read_bits macro
#[macro_export]
macro_rules! read_bits {
    // Base case: when the number of bits to read is 0, return 0
    ($data:expr, $offset:expr, 0, $type:ty) => {{
        0 as $type
    }};

    // Case for extracting a single bit as bool
    ($data:expr, $offset:expr, 1usize, bool) => {
        $data[$offset]
    };

    // Recursive case: when there are bits left to read
    ($data:expr, $offset:expr, $bits:expr, $type:ty) => {
        $data[$offset..$offset + $bits].load::<$type>();
    };
}

// Define the BitRead trait
pub trait BitRead {
    fn read_from(data: &[u8]) -> Result<Self, ReadError>
    where
        Self: Sized;
}

// Define the ReadError type
#[derive(Debug)]
pub enum ReadError {
    // Different kinds of errors
}
