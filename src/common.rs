use std::str::FromStr;
use std::fmt;
use std::error;
use simple;

#[derive(Clone, PartialEq, Debug)]
pub struct AddrError {
    pub msg: String,
}

impl fmt::Display for AddrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl error::Error for AddrError {
    fn description(&self) -> &str {
        self.msg.as_ref()
    }
}

/// Email address data structure
///
/// Represents an email address, right now just the `name@domain` portion.
///
/// # Example
///
/// ```
/// use emailaddress::EmailAddress;
/// let email = match EmailAddress::new("someone@example.com") {
///     Ok(addr) => addr,
///     Err(e) => panic!("Error parsing address, error was {}", e),
/// };
/// assert_eq!(&email.local, "someone");
/// assert_eq!(&email.domain, "example.com");
/// ```
#[derive(Clone, PartialEq, Debug)]
pub struct EmailAddress {
    pub local: String,
    pub domain: String,
}

impl EmailAddress {
    pub fn new(string: &str) -> Result<EmailAddress, AddrError> {
        simple::parse(string)
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.local, self.domain)
    }
}

impl FromStr for EmailAddress {
    type Err = AddrError;
    fn from_str(string: &str) -> Result<EmailAddress, AddrError> {
        simple::parse(string)
    }
}
