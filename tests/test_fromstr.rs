extern crate emailaddress;
use self::emailaddress::Address;

#[test]
fn test_from_str() {
    let addr: Option<Address> = from_str("someone@example.com");
    assert_eq!(
            addr.unwrap(),
            Address::new("someone".to_string(), "example.com".to_string())
        );
}
