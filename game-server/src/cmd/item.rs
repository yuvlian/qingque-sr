use crate::packet::PacketSink;
use sr_proto::GetBagScRsp;
use sr_proto::cmd::GET_BAG_SC_RSP;

pub async fn get_bag(_req: &[u8], sink: &mut PacketSink) {
    sink.push_message(GET_BAG_SC_RSP, GetBagScRsp::default());
}
