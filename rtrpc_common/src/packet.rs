use bytes::{Bytes, BytesMut};
use tokio::codec::*;

/// The unit of information between client and server
#[derive(Debug, PartialEq, Clone)]
pub struct Packet {
    pub id: u32,
    pub payload: Bytes,
}

impl From<BytesMut> for Packet {
    fn from(frame: BytesMut) -> Self {
        let frame_data = frame.freeze();
        use bytes::Buf;
        use std::io::Cursor;
        Packet {
            id: Cursor::new(&*frame_data).get_u32_be(),
            payload: frame_data.slice_from(4),
        }
    }
}

impl Into<Bytes> for Packet {
    fn into(self) -> Bytes {
        let mut buf = BytesMut::new();
        use bytes::BufMut;
        buf.reserve(4 + self.payload.len());
        buf.put_u32_be(self.id);
        buf.extend_from_slice(&self.payload);
        buf.freeze()
    }
}

#[test]
fn packet_encode() {
    let packet = Packet {
        id: 0xDEADBEEF,
        payload: Bytes::from(&b"Test"[..]),
    };
    let encoded: Bytes = packet.clone().into();
    let mut bytes = BytesMut::new();
    bytes.extend_from_slice(&encoded);
    assert_eq!(packet, Packet::from(bytes))
}

pub struct PacketCodec {
    inner: LengthDelimitedCodec,
}
impl PacketCodec {
    pub fn new() -> Self {
        PacketCodec {
            inner: length_delimited::Builder::new()
                .length_adjustment(4)
                .new_codec(),
        }
    }
}

impl Decoder for PacketCodec {
    type Item = Packet;
    type Error = <LengthDelimitedCodec as Decoder>::Error;
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Packet>, Self::Error> {
        let inner_result = self.inner.decode(src);
        inner_result.map(|opt| opt.map(|frame| Packet::from(frame)))
    }
}

impl Encoder for PacketCodec {
    type Item = Packet;
    type Error = <LengthDelimitedCodec as Encoder>::Error;
    fn encode(&mut self, data: Packet, dst: &mut BytesMut) -> Result<(), Self::Error> {
        self.inner.encode(data.into(), dst)
    }
}
