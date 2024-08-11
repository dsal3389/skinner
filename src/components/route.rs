use super::interface::Interface;
use std::net;

#[derive(Debug)]
pub struct Route<'a> {
    destination: net::IpAddr,
    interface: Option<&'a Interface>,
}

impl<'a> Route<'a> {
    pub fn new(destination: net::IpAddr, interface: Option<&'a Interface>) -> Self {
        Route {
            destination,
            interface,
        }
    }
}
