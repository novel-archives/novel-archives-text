#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate derive_getters;

pub mod parser;
mod part;
mod text;

pub use part::*;
pub use text::*;
