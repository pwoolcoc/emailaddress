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

## Simple parsing

The "simple" parsing algorithm is this:

  # take the last occurrence of the '@' symbol
  # everything to the right of it is the domain part
  # everything to the left of it is the local port

"WHAT??!!"

Yes, that's it. Not really a parser. Not really an algorithm (at least,
not much of one). But for reasons why you would want to use it, see
[here][1] or just google/duckduckgo/startpage for "email address RFC".

1: http://girders.org/blog/2013/01/31/dont-rfc-validate-email-addresses/

