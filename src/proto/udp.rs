use crate::proto::udp::inbound::UdpResponsePacket;
use crate::proto::udp::outbound::UdpControlPacket;
use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

pub mod inbound;
pub mod outbound;

/// The tokio codec for UDP traffic to and from the roboRIO
pub struct DsUdpCodec;

impl Decoder for DsUdpCodec {
    type Item = UdpResponsePacket;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match UdpResponsePacket::decode(src) {
            Ok(packet) => {
                Ok(Some(packet))
            }
            // In other Decoder implementations, the error is checked and if it was due
            // to a lack of data, Ok(None) is returned.
            // This implementation does it differently due to a bug in UDPFramed assuming there's nothing left
            // to decode if None is ever returned.
            Err(e) => Err(e),
        }
    }
}

impl Encoder<UdpControlPacket> for DsUdpCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: UdpControlPacket, dst: &mut BytesMut) -> Result<(), Self::Error> {
        println!("a");
        dst.extend(item.encode());

        Ok(())
    }
}
