# Email Address type for Rust

[![Build Status](https://travis-ci.org/pwoolcoc/emailaddress.svg?branch=master)](https://travis-ci.org/pwoolcoc/emailaddress)

This crate implements email address parsing for Rust, as well as an `Address` type, 
so you can stop stringly-typing your email addresses.

```
use emailaddress::simple;

fn main() {
    let email = simple::parse("someone@example.com");
    assert_eq!(email.local, "someone".to_string());
    assert_eq!(email.domain, "example.com".to_string());
}

// or with from_str:

use emailaddress::Address;

fn main() {
  let email: Option<Address> = from_str("someone@example.com");
  assert_eq!(email.local, "someone".to_string());
  assert_eq!(email.domain, "example.com".to_string());
}

```

