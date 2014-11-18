#![feature(globs)]
pub use common::EmailAddress;

mod common;
mod simple;

#[cfg(feature = "rfc5322")]
mod rfc5322;

#[cfg(feature = "rfc6531")]
mod rfc6531;
