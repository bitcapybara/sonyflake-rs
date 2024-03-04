//! A distributed unique ID generator inspired by [Twitter's Snowflake].
//!
//! This is a Rust implementation of the original [sony/sonyflake], which is written in Go.
//!
//! ## Quickstart
//!
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! sonyflake = "0.1"
//! ```
//!
//! Use the library like this:
//!
//! ```
//! use sonyflake::Sonyflake;
//!
//! let mut sf = Sonyflake::new(1);
//! let next_id = sf.next_id();
//! println!("{}", next_id);
//! ```
//!
//! ## Concurrent use
//!
//! Sonyflake is threadsafe. `clone` it before moving to another thread:
//! ```
//! use sonyflake::Sonyflake;
//! use std::thread;
//!
//! let sf = Sonyflake::new(1);
//!
//! let mut children = Vec::new();
//! for _ in 0..10 {
//!     let mut thread_sf = sf.clone();
//!     children.push(thread::spawn(move || {
//!         println!("{}", thread_sf.next_id());
//!     }));
//! }
//!
//! for child in children {
//!     child.join().unwrap();
//! }
//! ```
//!
//! [sony/sonyflake]: https://github.com/sony/sonyflake
//! [Twitter's Snowflake]: https://blog.twitter.com/2010/announcing-snowflake
#![doc(html_root_url = "https://docs.rs/sonyflake/*")]

#[cfg(doctest)]
#[doc = include_str!("../README.md")]
pub struct ReadmeDoctests;

mod sonyflake;
#[cfg(test)]
mod tests;

pub use crate::sonyflake::*;
