use futures::{Future, Stream};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;
use tokio_io::io::copy;
use tokio_io::AsyncRead;

mod core;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "127.0.0.1:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle).unwrap();
    let server = listener.incoming().for_each(|(sock, _)| {
        let (reader, writer) = sock.split();
        let bytes_copied = copy(reader, writer);
        let handle_conn = bytes_copied
            .map(|amt| println!("wrote {:?} bytes", amt))
            .map_err(|err| println!("IO error {:?}", err));
        handle.spawn(handle_conn);
        Ok(())
    });
    core.run(server).unwrap();
}
