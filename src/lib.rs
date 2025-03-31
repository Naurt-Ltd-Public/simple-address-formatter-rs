#![doc = include_str!("../README.md")]
#![deny(missing_docs, unused_imports)]

mod address;
mod error;

pub use address::{SimpleAddressFormat, SimpleAddressFormatter};
pub use error::SimpleAddressError;
