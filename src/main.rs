#[macro_use]
extern crate log;

use std::str::FromStr;
use std::net::{UdpSocket, Ipv4Addr};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn listen_udp() -> Result<()> {
    info!("starting UDP socket");

    // Join the upnp multicast group 239.255.255.250:1900.
    // Let the system automatically choose an appropriate interface.
    let socket = UdpSocket::bind("0.0.0.0:1900")?;
    let ssdp_mcast_addr = Ipv4Addr::from_str("239.255.255.250").unwrap();
    socket.join_multicast_v4(&ssdp_mcast_addr, &Ipv4Addr::UNSPECIFIED)?;

    // Read a single message and exit.
    let mut buf = [0; 65507];
    let (amt, src) = socket.recv_from(&mut buf)?;
    debug!("{:?} bytes from {:?}", amt, src);
    let buf = &mut buf[..amt];
    debug!("{}", std::str::from_utf8(&buf)?);

    // TODO: Validate that it is a M-SEARCH message.

    // TODO: Reply with a valid response.
    // Example (from OpenWRT router running miniupnpd):
    // HTTP/1.1 200 OK
    // CACHE-CONTROL: max-age=120
    // ST: upnp:rootdevice
    // USN: uuid:<uuid>::upnp:rootdevice
    // EXT:
    // SERVER: <device_name>/19.07-SNAPSHOT UPnP/1.1 MiniUPnPd/2.2.0
    // LOCATION: http://<ip>/rootDesc.xml
    // OPT: "http://schemas.upnp.org/upnp/1/0/"; ns=01
    // 01-NLS: 1626902704
    // BOOTID.UPNP.ORG: 1626902704
    // CONFIGID.UPNP.ORG: 1337
    Ok(())
}

fn main() -> Result<()> {
    env_logger::builder().filter_level(log::LevelFilter::Debug).init();
    listen_udp()?;
    // TODO: Serve a rootDesc.xml describing available services.
    // TODO: Execute requested actions.
    Ok(())
}
