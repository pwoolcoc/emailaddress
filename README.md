# Email Address type for Rust

[![Build Status](https://travis-ci.org/pwoolcoc/emailaddress-rs.svg?branch=master)](https://travis-ci.org/pwoolcoc/emailaddress-rs)

This crate implements email address parsing for Rust, as well as an `EmailAddress`
type, so you can stop stringly-typing your email addresses.

```
use emailaddress::EmailAddress;

fn main() {
    let email = EmailAddress::new("someone@example.com").unwrap();
    assert_eq!(&email.local, "someone"); 
    assert_eq!(&email.domain, "example.com");
}

// or with from_str:

use emailaddress::EmailAddress;

fn main() {
  let email = from_str::<EmailAddress>("someone@example.com").unwrap();
  assert_eq!(
    email,
    EmailAddress {
        local: "someone".to_string(),
        domain: "example.com".to_string()
    }
  );
}

```

## Parsing

There are (erm..."will be") 3 different parsing algorithms. "simple",
"rfc5322" and "rfc6531".  Currently only "simple" is fully implemented.

## Simple parsing

The "simple" parsing algorithm is this:

  * take the last occurrence of the '@' symbol
  * everything to the right of it is the domain part
  * everything to the left of it is the local port

"WHAT??!!"

Yes, that's it. Not really a parser. Not much of an algorithm. But for reasons
why you would want to use it, see
http://girders.org/blog/2013/01/31/dont-rfc-validate-email-addresses/ or just
google/duckduckgo/startpage for "email address RFC".



