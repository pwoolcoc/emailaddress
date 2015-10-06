use std::str::FromStr;
use std::fmt;
use std::error;
use simple::parse;

#[derive(Clone, PartialEq, Debug)]
pub struct AddrError {
    pub msg: String
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

#[derive(Clone, PartialEq, Debug)]
pub struct EmailAddress {
    pub local: String,
    pub domain: String,
}

impl EmailAddress {
    pub fn new(string: &str) -> EmailAddress {
        parse(string).unwrap()
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
        parse(string)
    }
}

