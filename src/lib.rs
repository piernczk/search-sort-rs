//! An implementation of few searching and sorting algorithms.
//!
//! This crate is currently WIP, and supports only few of them.
//! Currently supported algorithms:
//! - [linear](search::linear) search
//! - [binary](search::binary) search
//! - [bubble](sort::bubble) sort
//! - [quick](sort::quick) sort
//! 
//! # Quick example
//! ```no_run
//! use search_sort::{search, sort};
//! 
//! let mut slice = [5, 1, 91, -45, 11, 5];
//! sort::quick(&mut slice);
//! assert_eq!(slice, [-45, 1, 5, 5, 11, 91]);
//! 
//! assert_eq!(Some(2), search::binary_first(&slice, &5));
//! assert_eq!(None, search::binary_first(&slice, &42));
//! ```

pub mod search;
pub mod sort;
