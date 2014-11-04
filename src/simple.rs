use common::{Address};
use std::error::Error;

#[deriving(Show)]
pub struct SimpleError {
    msg: String
}

impl Error for SimpleError {
    fn description(&self) -> &str {
        self.msg.as_slice()
    }
}

/// Performs a dead-simple parse of an email address. 
pub fn parse(input: String) -> Result<Address, SimpleError> {
    if input.is_empty() {
        return Err(SimpleError { msg: "empty string is not valid".to_string() });
    }
    let parts: Vec<&str> = input.rsplitn(1, '@').collect();

    if parts[0].is_empty() {
        return Err(SimpleError { msg: "empty string is not valid for local part".to_string() });
    }

    if parts[1].is_empty() {
        return Err(SimpleError { msg: "empty string is not valid for domain part".to_string() });
    }

    Ok(Address::new(parts[1].to_string(), parts[0].to_string()))
}

#[cfg(test)]
mod test {
    use super::*;
    use common::{Address};

    #[test]
    fn test_simple_parse() {
        assert_eq!(
                parse("someone@example.com".to_string()).unwrap(),
                Address::new("someone".to_string(), "example.com".to_string())
            );
    }
}
