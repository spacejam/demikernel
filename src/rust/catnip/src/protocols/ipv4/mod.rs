pub mod checksum;

mod datagram;
mod peer;
mod endpoint;

pub use datagram::{
    Ipv4Datagram as Datagram, Ipv4DatagramMut as DatagramMut,
    Ipv4Header as Header, Ipv4HeaderMut as HeaderMut,
    Ipv4Protocol as Protocol, IPV4_HEADER_SIZE as HEADER_SIZE,
};
pub use peer::Ipv4Peer as Peer;
pub use endpoint::Ipv4Endpoint as Endpoint;

use crate::protocols::ethernet2;

pub fn new_datagram(payload_sz: usize) -> Vec<u8> {
    ethernet2::new_datagram(payload_sz + HEADER_SIZE)
}
