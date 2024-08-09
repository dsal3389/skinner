use super::interface::Interface;
use anyhow::Result;
use std::net;

#[derive(Debug)]
pub struct Route<'a> {
    destination: net::IpAddr,
    interface: Option<&'a Interface>,
}

impl<'a> Route<'a> {
    fn new(destination: net::IpAddr, interface: Option<&'a Interface>) -> Self {
        Route {
            destination,
            interface,
        }
    }

    // generate `n` number of routes, it is expected to get also the list of the
    // generated interfaces, to have valid routes next-hop and outgoing interface
    pub fn generaten(interfaces: &Vec<Interface>, count: u16) -> Result<Vec<Route>> {
        let mut routes = Vec::with_capacity(count as usize);
        let mut ten_address = 0x0a000000u32;

        for interface in interfaces {
            ten_address += 1;
            let address = net::Ipv4Addr::from(ten_address);
            routes.push(Route::new(net::IpAddr::V4(address), Some(&interface)));
        }
        Ok(routes)
    }
}
