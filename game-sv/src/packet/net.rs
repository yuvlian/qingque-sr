use byteorder::{BE, ByteOrder};
use sr_proto::prost::{DecodeError as ProtoDecodeError, Message};

const OVERHEAD: usize = 16;
const HEAD_MAGIC: [u8; 4] = [0x9D, 0x74, 0xC7, 0x14];
// not gonna decode or encode head so yeah
const HEAD_LEN: usize = 0;
pub const TAIL_MAGIC: [u8; 4] = [0xD7, 0xA1, 0x52, 0xC8];

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("head magic mismatch")]
    HeadMagicMismatch,
    #[error("tail magic mismatch")]
    TailMagicMismatch,
    #[error("input buffer is less than overhead, len: {0}, overhead: {1}")]
    InputLessThanOverhead(usize, usize),
    #[error("out of bounds ({0}/{1})")]
    OutOfBounds(usize, usize),
    // #[error("failed to decode PacketHead: {0}")]
    // HeadDecode(ProtoDecodeError),
    #[error("failed to decode body: {0}")]
    BodyDecode(ProtoDecodeError),
}

pub fn read_common_values(buf: &[u8]) -> Result<(u16, usize, usize), DecodeError> {
    if buf.len() < OVERHEAD {
        return Err(DecodeError::InputLessThanOverhead(buf.len(), OVERHEAD));
    }

    if &buf[0..4] != HEAD_MAGIC {
        return Err(DecodeError::HeadMagicMismatch);
    }

    let cmd_id = BE::read_u16(&buf[4..6]);
    let head_len = BE::read_u16(&buf[6..8]) as usize;
    let body_len = BE::read_u32(&buf[8..12]) as usize;
    let min_len = 4 + head_len + body_len;

    if min_len > buf.len() {
        Err(DecodeError::OutOfBounds(min_len, buf.len()))
    } else {
        Ok((cmd_id, head_len, body_len))
    }
}

pub struct NetPacket<T: Message + Default> {
    pub cmd_id: u16,
    pub body: T,
}

impl<T: Message + Default> NetPacket<T> {
    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        let (cmd_id, head_len, body_len) = read_common_values(buf)?;

        if &buf[(12 + head_len + body_len)..(16 + head_len + body_len)] != TAIL_MAGIC {
            return Err(DecodeError::TailMagicMismatch);
        }

        let body = T::decode(&buf[(12 + head_len)..(12 + head_len + body_len)])
            .map_err(DecodeError::BodyDecode)?;

        Ok(Self { cmd_id, body })
    }

    fn encode(&self) -> Box<[u8]> {
        let body_len = self.body.encoded_len();
        let total_len = OVERHEAD + HEAD_LEN + body_len;

        let mut buf = vec![0u8; total_len];
        (&mut buf[0..4]).copy_from_slice(&HEAD_MAGIC);

        BE::write_u16(&mut buf[4..6], self.cmd_id);
        BE::write_u16(&mut buf[6..8], HEAD_LEN as u16);
        BE::write_u32(&mut buf[8..12], body_len as u32);

        self.body
            .encode(&mut buf[(12 + HEAD_LEN)..(12 + HEAD_LEN + body_len)].as_mut())
            .unwrap();

        (&mut buf[(12 + HEAD_LEN + body_len)..(16 + HEAD_LEN + body_len)])
            .copy_from_slice(&TAIL_MAGIC);

        buf.into_boxed_slice()
    }
}
