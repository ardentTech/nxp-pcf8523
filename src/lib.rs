#![no_std]

pub mod registers;
pub(crate) mod bits;
pub mod driver;
pub mod datetime;
pub use driver::Pcf8523;