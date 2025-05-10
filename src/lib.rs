//! # multid
//! multidimensional (currently just 2) vectors parameterized by number of columns and rows
//!
//! includes some helpful iterators and an interface that gracefully handles boundaries
//!
//! requires `feature(generic_const_exprs)`
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
pub mod errors;
pub mod ix;
pub mod v;

pub use ix::BoundedIx2;
pub use ix::iterators;
pub use v::V2;
