//! tinystr is a small ASCII-only bounded length string representation.
//!
//! # Example
//!
//! ```
//! use tinystr::{TinyStr4, TinyStr8};
//!
//! fn main() {
//!     let s1: TinyStr4 = "tEsT".parse()
//!         .expect("Failed to parse.");
//!
//!     assert_eq!(s1, "tEsT");
//!     assert_eq!(s1.to_ascii_uppercase(), "TEST");
//!     assert_eq!(s1.to_ascii_lowercase(), "test");
//!     assert_eq!(s1.to_ascii_titlecase(), "Test");
//!     assert_eq!(s1.is_ascii_alphanumeric(), true);
//!
//!     let s2: TinyStr8 = "New York".parse()
//!         .expect("Failed to parse.");
//!
//!     assert_eq!(s2, "New York");
//!     assert_eq!(s2.to_ascii_uppercase(), "NEW YORK");
//!     assert_eq!(s2.to_ascii_lowercase(), "new york");
//!     assert_eq!(s2.is_ascii_alphanumeric(), false);
//! }
//! ```
mod helpers;
mod tinystr4;
mod tinystr8;
mod tinystr16;

pub use tinystr4::TinyStr4;
pub use tinystr8::TinyStr8;
pub use tinystr16::TinyStr16;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    InvalidSize,
    InvalidNull,
    NonAscii,
}
