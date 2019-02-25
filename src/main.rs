use futures::{Future, Stream};
use rtrpc_common::PacketCodec;
use tokio::codec::Framed;
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

mod core;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle).unwrap();
    let server = listener.incoming().for_each(|(sock, _)| {
        let (sink, stream) = Framed::new(sock, PacketCodec::new()).split();
        let process_packet = |a| {
            println!("{:?}", a);
            a
        };
        handle.spawn(stream.map(process_packet).forward(sink).then(|result| {
            match result {
                Ok(_) => println!("finished"),
                Err(e) => println!("error: {}", e),
            }
            Ok(())
        }));
        Ok(())
    });
    core.run(server).unwrap();
}
