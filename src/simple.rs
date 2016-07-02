use common::{EmailAddress, AddrError};

/// Performs a dead-simple parse of an email address. 
pub fn parse(input: &str) -> Result<EmailAddress, AddrError> {
    if input.is_empty() {
        return Err(AddrError { msg: "empty string is not valid".to_string() });
    }
    let parts: Vec<&str> = input.rsplitn(2, '@').collect();

    if parts.len() < 2 {
        return Err(AddrError { msg: "missing '@' character".to_string() });
    }

    if parts[0].is_empty() {
        return Err(AddrError { msg: "empty string is not valid for local part".to_string() });
    }

    if parts[1].is_empty() {
        return Err(AddrError { msg: "empty string is not valid for domain part".to_string() });
    }

    Ok(EmailAddress {
        local: parts[1].to_string(),
        domain: parts[0].to_string()
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{EmailAddress};

    #[test]
    fn test_simple_parse() {
        assert!(parse("f@a").is_ok());
        assert!(parse("foo").is_err());
        assert_eq!(
                parse("someone@example.com").unwrap(),
                EmailAddress {
                    local: "someone".to_string(),
                    domain: "example.com".to_string()
                }
            );
    }
}
