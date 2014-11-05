use common::{EmailAddress, AddrError};

pub fn parse(string: &str) -> Result<EmailAddress, AddrError> {
    Ok(EmailAddress {
        local: "test".to_string(),
        domain: "example.com".to_string()
    })
}

