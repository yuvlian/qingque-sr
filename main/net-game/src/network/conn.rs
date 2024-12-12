use bytes::{Buf, BytesMut};
use tokio::io::Result as TokioResult;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::network::packet::decode_bytes;
use crate::network::router::ping_pong;
use tracing::{error, info};

pub async fn handle_connection(mut socket: TcpStream) -> TokioResult<()> {
    let mut buffer = BytesMut::with_capacity(1024);
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
            .position(|window| window == 0xD7A152C8_u32.to_be_bytes())
        {
            if position + 4 <= buffer.len() {
                let complete_message = &buffer[..position + 4];

                match decode_bytes(complete_message).await {
                    Ok((cmd, body)) => {
                        info!("{} -> {:?}", cmd, body);

                        let response = ping_pong(cmd, body).await;
                        if !response.is_empty() {
                            info!("{:?} -> {}", response, cmd);
                            socket
                                .write_all(&response)
                                .await
                                .expect("Failed to write to socket");
                        }
                    }
                    Err(e) => {
                        error!("Decoding error: {}", e);
                    }
                }

                buffer.advance(position + 4);
            } else {
                break;
            }
        }
    }
}
