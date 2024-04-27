#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod bands;
mod haar;
mod sampling;

pub use bands::Bands;
