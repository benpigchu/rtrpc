use futures::{Future, Sink, Stream};
use rtrpc_common::*;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio::codec::Framed;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Handle;

/// A client used to invoke rpc request
pub struct Client {
    handle: Handle,
    addr: SocketAddr,
}

impl Client {
    /// Create a client with the handle and address of the server
    pub fn new(handle: Handle, addr: SocketAddr) -> Self {
        Client { handle, addr }
    }
    /// Find the shortest path from the start point to the end point
    pub fn shortest_path(
        &self,
        graph: &Graph,
        start: &str,
        end: &str,
    ) -> impl Future<Item = Result<Option<Vec<String>>, NegativeCycle>, Error = Error> {
        let graph = graph.clone();
        let start = String::from(start);
        let end = String::from(end);
        // currently we create one connection for each request.
        TcpStream::connect(&self.addr, &self.handle).and_then(move |sock| {
            let (sink, stream) = Framed::new(sock, PacketCodec::new()).split();
            let send = sink
                .send(Packet {
                    id: 0xDEADBEEF,
                    payload: encode_request(&graph, &start, &end),
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
            send.join(recieve).and_then(|(_, result)| Ok(result))
        })
    }
}
