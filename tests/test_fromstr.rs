extern crate emailaddress;
use self::emailaddress::EmailAddress;

#[test]
fn test_from_str() {
    let addr: Option<EmailAddress> = from_str("someone@example.com");
    assert_eq!(
            addr.unwrap(),
            EmailAddress {
                local: "someone".to_string(),
                domain: "example.com".to_string()
            }
        );
}
