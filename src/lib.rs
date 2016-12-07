//! This crate provides a simple data type for representing an email
//! address. Currently it provides simple, non-RFC compliant parsing
//! of `name@domain` addresses, though RFC-compliant parsing is planned
#![cfg_attr(feature = "ser", feature(proc_macro))]

#[cfg(feature = "ser")]
#[macro_use] extern crate serde_derive;

pub use common::{EmailAddress, AddrError};

mod common;
mod simple;
// mod rfc5322;
// mod rfc6531;


