use bytes::Bytes;
use futures::{Future, Sink, Stream};
use rtrpc_common::{Packet, PacketCodec};
use tokio::codec::Framed;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "127.0.0.1:12345".parse().unwrap();
    let client = TcpStream::connect(&addr, &handle)
        .and_then(|sock| {
            let (sink, stream) = Framed::new(sock, PacketCodec::new()).split();
            let send = sink
                .send(Packet {
                    id: 0xDEADBEEF,
                    payload: Bytes::from(&b"Test"[..]),
                })
                .and_then(|sink| sink.flush());
            let recieve = stream.into_future().map_err(|(err, _)| err);
            send.join(recieve).then(|result| {
                match result {
                    Ok((_, (packet, _))) => println!("finished: {:?}", packet),
                    Err(e) => println!("error: {}", e),
                }
                Ok(())
            })
        })
        .map_err(|err| {
            println!("connection error = {:?}", err);
        });
    core.run(client).unwrap();
}
