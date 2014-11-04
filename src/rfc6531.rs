use common::{Address, AddrError};

pub fn parse(string: &str) -> Result<Address, AddrError> {
    Ok(Address::new("test".to_string(), "example.com".to_string()))
}

