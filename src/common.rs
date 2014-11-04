use std::from_str::FromStr;
use simple::parse;

#[deriving(Show, PartialEq)]
pub struct Address {
    pub local: String,
    pub domain: String,
}

impl Address {
    pub fn new(local: String, domain: String) -> Address {
        Address {
            local: local,
            domain: domain,
        }
    }
}

impl FromStr for Address {
    fn from_str(string: &str) -> Option<Address> {
        match parse(string.to_string()) {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

