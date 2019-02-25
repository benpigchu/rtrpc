use futures::{Future, Sink, Stream};
use rtrpc_common::*;
use std::io::{Error, ErrorKind};
use tokio::codec::Framed;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

fn main() {
    let graph = Graph::from_edges(&[
        ("a", "b", 1.0),
        ("b", "c", 2.0),
        ("a", "c", 4.0),
        ("d", "c", 3.0),
        ("c", "e", -4.0),
        ("c", "e", 4.0),
        ("f", "g", 4.0),
        ("g", "f", -6.0),
        ("f", "h", 2.0),
        ("h", "f", 2.0),
    ]);
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "127.0.0.1:12345".parse().unwrap();
    let client = TcpStream::connect(&addr, &handle)
        .and_then(|sock| {
            let (sink, stream) = Framed::new(sock, PacketCodec::new()).split();
            let send = sink
                .send(Packet {
                    id: 0xDEADBEEF,
                    payload: encode_request(&graph, "a", "e"),
                })
                .and_then(|sink| sink.flush());
            let recieve = stream
                .into_future()
                .map_err(|(err, _)| err)
                .and_then(|(packet, _)| {
                    if let Some(Packet { id, payload }) = packet {
                        if id != 0xDEADBEEF {
                            return Err(Error::new(ErrorKind::InvalidData, "Invalid packet id."));
                        }
                        decode_respond(payload).ok_or(Error::new(
                            ErrorKind::InvalidData,
                            "Can not parse respond packet.",
                        ))
                    } else {
                        Err(Error::new(ErrorKind::InvalidData, "No packet returned."))
                    }
                });
            send.join(recieve).then(|result| {
                match result {
                    Ok((_, result)) => println!("finished: {:?}", result),
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
