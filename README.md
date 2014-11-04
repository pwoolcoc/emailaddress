# Email Address type for Rust

This crate implements email address parsing for Rust, as well as an `Address` type, 
so you can stop stringly-typing your email addresses.

```
use emailaddress::simple;

fn main() {
    let email = simple::parse("someone@example.com");
    assert_eq!(email.local, "someone".to_string());
    assert_eq!(email.domain, "example.com".to_string());
}
```


