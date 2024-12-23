pub async fn decode_bytes(buffer: &[u8]) -> Result<(u16, &[u8]), &str> {
    if buffer.len() < 16 {
        return Err("Byte array is too short");
    }

    let head_magic = u32::from_be_bytes(buffer[0..4].try_into().expect("Invalid head magic slice"));

    if head_magic != 0x9D74C714 {
        return Err("Invalid head magic value");
    }

    let cmd = u16::from_be_bytes(buffer[4..6].try_into().expect("Invalid command slice"));

    let head_size =
        u16::from_be_bytes(buffer[6..8].try_into().expect("Invalid head size slice")) as usize;

    let body_size =
        u32::from_be_bytes(buffer[8..12].try_into().expect("Invalid body size slice")) as usize;

    let head_start = 12;

    let head_end = head_start + head_size;

    if head_end > buffer.len() {
        return Err("Head data exceeds byte array length");
    }

    let _head_data = &buffer[head_start..head_end];

    let body_start = head_end;

    let body_end = body_start + body_size;

    if body_end + 4 > buffer.len() {
        return Err("Body data exceeds byte array length");
    }

    let body_data = &buffer[body_start..body_end];

    let tail_magic = u32::from_be_bytes(
        buffer[body_end..body_end + 4]
            .try_into()
            .expect("Invalid tail magic slice"),
    );

    if tail_magic != 0xD7A152C8 {
        return Err("Invalid tail magic value");
    }

    Ok((cmd, body_data))
}

pub fn encode_packet(cmd_id: u16, data: Vec<u8>) -> Vec<u8> {
    let packet_len = 12 + data.len() + 4;
    let mut buffer = Vec::with_capacity(packet_len);

    buffer.extend_from_slice(&0x9D74C714u32.to_be_bytes());
    buffer.extend_from_slice(&cmd_id.to_be_bytes());
    buffer.extend_from_slice(&0u16.to_be_bytes());
    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buffer.extend_from_slice(&data);
    buffer.extend_from_slice(&0xD7A152C8u32.to_be_bytes());

    buffer
}
