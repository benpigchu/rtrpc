use futures::{Future, Stream};
use rtrpc_common::PacketCodec;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use tokio::codec::Framed;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Handle;

/// A server used to recieve rpc request
pub struct Server {
    handle: Handle,
    addr: SocketAddr,
}

impl Server {
	/// Create a server with the handle and address used to listen
    pub fn new(handle: Handle, addr: SocketAddr) -> Self {
        Server { handle, addr }
    }
	/// Consume the server and returns the serving Future,
	/// fail when failed to listen the Ip address
    pub fn serve(self) -> Result<impl Future<Item = (), Error = Error>, Error> {
        let future = TcpListener::bind(&self.addr, &self.handle)?
            .incoming()
            .for_each(move |(sock, _)| {
                let (sink, stream) = Framed::new(sock, PacketCodec::new()).split();
                let process_packet = |packet| {
                    crate::core::process_packet(packet).ok_or(Error::new(
                        ErrorKind::InvalidData,
                        "Can not parse request packet.",
                    ))
                };
                self.handle.spawn(
                    stream
                        .and_then(process_packet)
                        .forward(sink)
                        .then(|result| {
                            match result {
                                Ok(_) => {}
                                Err(e) => println!("error: {}", e),
                            }
                            Ok(())
                        }),
                );
                Ok(())
            });
        Ok(future)
    }
}
