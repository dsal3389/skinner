use std::net::IpAddr;

#[derive(Debug)]
pub struct Interface {
    name: String,
    address: IpAddr,
}

impl Interface {
    pub const JINAJ_NAME_TEMP: &'static str = "interface name";

    pub fn new(name: String, address: IpAddr) -> Self {
        Self { name, address }
    }
}
