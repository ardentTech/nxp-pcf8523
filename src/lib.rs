#![no_std]

pub(crate) mod bits;
pub mod datetime;
pub mod driver;
pub mod registers;
pub mod typedefs;

pub use driver::Pcf8523;
