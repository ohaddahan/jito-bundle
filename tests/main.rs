#![allow(clippy::unwrap_used, reason = "test code")]
#![allow(clippy::expect_used, reason = "test code")]
#![allow(clippy::panic, reason = "test code")]
#![allow(clippy::print_stdout, reason = "tests print tx signatures to stdout")]

#[cfg(feature = "live-tests")]
mod build;
pub mod common;
mod offline;
#[cfg(feature = "live-tests")]
mod send;
#[cfg(feature = "live-tests")]
mod simulate;
