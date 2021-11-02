pub use error::Error;

pub mod error;
pub mod establishment;
pub mod protocol;
pub mod support;
pub mod tcp;

pub const ADDRESSING_SIZE: u32 = usize::BITS;