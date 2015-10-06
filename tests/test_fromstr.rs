extern crate emailaddress;
use self::emailaddress::{EmailAddress, AddrError};
use std::str::{FromStr};

#[test]
fn test_from_str() {
    let addr: Result<EmailAddress, AddrError>  = FromStr::from_str("someone@example.com");
    assert_eq!(
            addr.unwrap(),
            EmailAddress {
                local: "someone".to_string(),
                domain: "example.com".to_string()
            }
        );
}
