use gateway_protocol::{IncomingPacket, OutgoingPacket};
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

/// Encode an outgoing packet to be sent to the gateway.
#[wasm_bindgen]
pub fn encode_outgoing(content: &str) -> Result<js_sys::Uint8Array, String> {
    let packet: OutgoingPacket =
        serde_json::from_str(content).map_err(|_| "failed to parse packet")?;
    let packet = bincode::serialize(&packet).map_err(|_| "failed to binary-encode packet")?;
    Ok(Uint8Array::from(&packet[..]))
}

/// Decode an incoming packet from the gateway.
#[wasm_bindgen]
pub fn decode_incoming(data: js_sys::Uint8Array) -> Result<String, String> {
    let data = data.to_vec();
    let packet: IncomingPacket =
        bincode::deserialize(&data).map_err(|_| "failed to binary-decode packet")?;
    Ok(serde_json::to_string(&packet).map_err(|_| "failed to serialize packet")?)
}
