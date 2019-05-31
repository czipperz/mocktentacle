#![feature(fn_traits)]
#![feature(unboxed_closures)]

pub use mocktopus::macros::*;

mod expected_call;
pub use crate::expected_call::*;

mod call_glue;
mod verify;

mod mock;
pub use crate::mock::*;
