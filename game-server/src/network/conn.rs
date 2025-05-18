use bytes::{Buf, BytesMut};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::handlers::check_dummy;
use crate::network::packet::{TAIL_MAGIC_BYTES, decode_bytes, encode_packet_raw};
use crate::network::router::ping_pong;
use tracing::info;

pub async fn handle_connection(mut socket: TcpStream) -> tokio::io::Result<()> {
    let mut buffer = BytesMut::with_capacity(1200);
    let mut temp_buffer = [0u8; 1024];

    loop {
        let n = socket
            .read(&mut temp_buffer)
            .await
            .expect("Failed to read from socket");

        if n == 0 {
            info!("Connection closed.");
            return Ok(());
        }

        buffer.extend_from_slice(&temp_buffer[..n]);

        while let Some(position) = buffer
            .windows(4)
            .position(|window| window == TAIL_MAGIC_BYTES)
        {
            if position + 4 <= buffer.len() {
                let complete_message = &buffer[..position + 4];
                let (cmd, body) = decode_bytes(complete_message);

                info!("Received: {}", cmd);

                let response = match check_dummy(cmd) {
                    Some(dummy_cmd) => encode_packet_raw(dummy_cmd, Vec::new()),
                    _ => ping_pong(cmd, body).await,
                };

                if !response.is_empty() {
                    info!("Handled: {}", cmd);
                    socket
                        .write_all(&response)
                        .await
                        .expect("Failed to write to socket");
                }
                buffer.advance(position + 4);
            } else {
                break;
            }
        }
    }
}
