use std::str::FromStr;
use std::fmt::{Show, Formatter, FormatError};
use std::error::Error;
use simple::parse;

#[deriving(Show)]
pub struct AddrError {
    pub msg: String
}

impl Error for AddrError {
    fn description(&self) -> &str {
        self.msg.as_slice()
    }
}

#[deriving(PartialEq)]
pub struct EmailAddress {
    pub local: String,
    pub domain: String,
}

impl EmailAddress {
    pub fn new(string: &str) -> EmailAddress {
        parse(string).unwrap()
    }
}

impl Show for EmailAddress {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "{}@{}", self.local, self.domain)
    }
}

impl FromStr for EmailAddress {
    fn from_str(string: &str) -> Option<EmailAddress> {
        match parse(string) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

