#[deriving(Show, PartialEq)]
pub struct Address {
    local: String,
    domain: String,
}

impl Address {
    pub fn new(local: String, domain: String) -> Address {
        Address {
            local: local,
            domain: domain,
        }
    }
}
