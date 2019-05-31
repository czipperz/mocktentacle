//! Mocktentacle is a mocking framework designed to mock functions.
//!
//! ```rust
//! #[cfg(test)]
//! use mocktentacle::mockable;
//!
//! #[cfg_attr(test, mockable)]
//! fn f() -> i32 {
//!     0
//! }
//!
//! #[cfg(test)]
//! mod tests {
//!     use super::*;
//!     use mocktentacle::Mock;
//!
//!     #[test]
//!     fn test_mocktentacle() {
//!         let mut mock = Mock::new();
//!         mock.expect(f).answer(|| 1);
//!
//!         assert_eq!(f(), 1);
//!
//!         mock.verify();
//!     }
//! }
//! ```
//!
//! # Setup
//!
//! Include mocktentacle as a dependency only on development builds (`cargo
//! test`) put the following in Cargo.toml:
//!
//! ```text
//! [dev-dependencies]
//! mocktentacle = "0.1.0"
//! ```
//!
//! Annotate functions or `impl` blocks with `#[cfg_attr(test, mockable)]`.
//!
//! Use [`Mock`] inside tests to mock specific functions.
//!
//! [`Mock`]: struct.Mock.html

#![feature(fn_traits)]
#![feature(unboxed_closures)]

pub use mocktopus::macros::*;

mod expected_call;
pub use crate::expected_call::*;

mod call_glue;
mod verify;

mod mock;
pub use crate::mock::*;
