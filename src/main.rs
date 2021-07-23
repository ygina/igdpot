#[macro_use]
extern crate log;

use std::io;
use std::str::FromStr;
use std::net::{UdpSocket, Ipv4Addr};

fn listen_udp() -> io::Result<()> {
    info!("starting UDP socket");

    // Join the upnp multicast group 239.255.255.250:1900.
    // Let the system automatically choose an appropriate interface.
    let socket = UdpSocket::bind("0.0.0.0:1900")?;
    let ssdp_mcast_addr = Ipv4Addr::from_str("239.255.255.250").unwrap();
    socket.join_multicast_v4(&ssdp_mcast_addr, &Ipv4Addr::UNSPECIFIED)?;

    // Read a single message and exit.
    let mut buf = Vec::new();
    let (amt, src) = socket.recv_from(&mut buf)?;
    debug!("{:?} bytes from {:?}", amt, src);
    Ok(())
}

fn main() -> io::Result<()> {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    listen_udp()?;
    Ok(())
}
