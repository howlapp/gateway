//! # gateway_protocol
//! This crate defines the gateway protocol and its various types.
//!
//! ## Gateway Protocol
//! The gateway protocol is a simple, stateless, protocol that allows
//! clients to connect to a service and receive a websocket connection.

use serde::{Deserialize, Serialize};

/// An incoming packet.
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomingPacket {
    op: u16,
    seq: Option<u64>,
    d: Option<IncomingPayload>,
}

/// An outgoing packet.
#[derive(Debug, Serialize, Deserialize)]
pub struct OutgoingPacket {
    op: u16,
    seq: Option<u64>,
    d: Option<OutgoingPayload>,
}

/// Contains the enumerations of possible incoming packet payloads, based on the packet's opcode.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IncomingPayload {}

/// Contains the enumerations of possible outgoing packet payloads, based on the packet's opcode.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OutgoingPayload {}
