use crate::packet::{PacketSink, check_dummy, handle_packet};
use amia_packet::net_packet::NetPacket;
use tokio::net::TcpStream;

pub async fn handle_connection(mut stream: TcpStream) -> tokio::io::Result<()> {
    let mut packet_sink = PacketSink::with_capacity(5);

    loop {
        packet_sink.clear();

        let packet = NetPacket::read(&mut stream).await?;
        let req_cmd = packet.cmd;
        let req_body = packet.body;

        match check_dummy(req_cmd) {
            Some(dummy_cmd) => packet_sink.push_raw(dummy_cmd, Vec::new()),
            _ => handle_packet(req_cmd, &req_body, &mut packet_sink).await,
        }

        packet_sink.write_all(&mut stream).await?;
    }
}
