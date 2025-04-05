mod control;
mod net;

// thx xeon

pub use control::{CONTROL_PACKET_SIZE, ControlPacket, ControlPacketType};
pub use net::{DecodeError, NetPacket, TAIL_MAGIC as NET_PACKET_TAIL_MAGIC, read_common_values};
