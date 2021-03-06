// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use crate::{
    prelude::*,
    protocols::{icmpv4, tcp, udp},
};
use std::{
    cell::RefCell,
    fmt::{Debug, Formatter, Result as FmtResult},
    rc::Rc,
};

pub enum Event {
    Transmit(Rc<RefCell<Vec<u8>>>),
    Icmpv4Error {
        id: icmpv4::ErrorId,
        next_hop_mtu: u16,
        context: Vec<u8>,
    },
    UdpDatagramReceived(udp::Datagram),
    IncomingTcpConnection(tcp::ConnectionHandle),
    TcpBytesAvailable(tcp::ConnectionHandle),
    TcpConnectionClosed {
        handle: tcp::ConnectionHandle,
        error: Option<Fail>,
    },
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Event::")?;
        match self {
            Event::Transmit(bytes) => {
                write!(f, "Transmit {{ ")?;
                let bytes = bytes.borrow();
                match tcp::Segment::decode(&bytes) {
                    Ok(s) => write!(f, "{:?}", s)?,
                    _ => write!(f, "{:?}", bytes)?,
                }
                write!(f, " }}")?;
            }
            Event::Icmpv4Error {
                id,
                next_hop_mtu,
                context,
            } => write!(
                f,
                "Icmpv4Error {{ id: {:?}, next_hop_mtu: {:?}, context: {:?} \
                 }}",
                id, next_hop_mtu, context
            )?,
            Event::UdpDatagramReceived(datagram) => {
                write!(f, "UdpDatagramReceived({:?})", datagram)?
            }
            Event::IncomingTcpConnection(handle) => {
                write!(f, "IncomingTcpConnection({})", handle)?
            }
            Event::TcpBytesAvailable(handle) => {
                write!(f, "TcpBytesAvailable({})", handle)?
            }
            Event::TcpConnectionClosed { handle, error } => write!(
                f,
                "TcpConnectionClosed {{ handle: {:?}, error: {:?} }}",
                handle, error
            )?,
        }

        Ok(())
    }
}
