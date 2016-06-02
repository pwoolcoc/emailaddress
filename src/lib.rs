//! This crate provides a simple data type for representing an email
//! address. Currently it provides simple, non-RFC compliant parsing
//! of `name@domain` addresses, though RFC-compliant parsing is planned

pub use common::{EmailAddress, AddrError};

mod common;
mod simple;
// mod rfc5322;
// mod rfc6531;


