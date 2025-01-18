const HEAD_MAGIC: u32 = 0x9D74C714;
const TAIL_MAGIC: u32 = 0xD7A152C8;
const HEAD_MAGIC_BYTES: [u8; 4] = [0x9D, 0x74, 0xC7, 0x14];
pub const TAIL_MAGIC_BYTES: [u8; 4] = [0xD7, 0xA1, 0x52, 0xC8];

pub fn decode_bytes(buffer: &[u8]) -> (u16, &[u8]) {
    if buffer.len() < 16 {
        panic!("Byte array is too short");
    }

    let head_magic = u32::from_be_bytes(buffer[0..4].try_into().expect("Invalid head magic slice"));

    if head_magic != HEAD_MAGIC {
        panic!("Invalid head magic value");
    }

    let cmd = u16::from_be_bytes(buffer[4..6].try_into().expect("Invalid command slice"));

    let head_size =
        u16::from_be_bytes(buffer[6..8].try_into().expect("Invalid head size slice")) as usize;

    let body_size =
        u32::from_be_bytes(buffer[8..12].try_into().expect("Invalid body size slice")) as usize;

    let head_start = 12;

    let head_end = head_start + head_size;

    if head_end > buffer.len() {
        panic!("Head data exceeds byte array length");
    }

    let _head_data = &buffer[head_start..head_end];

    let body_start = head_end;

    let body_end = body_start + body_size;

    if body_end + 4 > buffer.len() {
        panic!("Body data exceeds byte array length");
    }

    let body_data = &buffer[body_start..body_end];

    let tail_magic = u32::from_be_bytes(
        buffer[body_end..body_end + 4]
            .try_into()
            .expect("Invalid tail magic slice"),
    );

    if tail_magic != TAIL_MAGIC {
        panic!("Invalid tail magic value");
    }

    (cmd, body_data)
}

pub fn encode_packet(cmd_id: u16, data: Vec<u8>) -> Vec<u8> {
    let packet_len = 12 + data.len() + 4;
    let mut buffer = Vec::with_capacity(packet_len);

    buffer.extend_from_slice(&HEAD_MAGIC_BYTES);
    buffer.extend_from_slice(&cmd_id.to_be_bytes());
    buffer.extend_from_slice(&0u16.to_be_bytes());
    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buffer.extend_from_slice(&data);
    buffer.extend_from_slice(&TAIL_MAGIC_BYTES);

    buffer
}
