//! # multid
//! multidimensional (currently just 2) vectors parameterized by number of columns and rows
//!
//! includes some helpful iterators and getters/setters, with an interface that gracefully
//! handles boundaries
//!
//! requires `feature(generic_const_exprs)`
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
pub mod errors;
pub mod ix;
pub mod v;

pub use ix::Ix2;
pub use v::V2;
