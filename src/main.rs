mod core;
mod server;
use server::*;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "127.0.0.1:12345".parse().expect("Invalid IP address");
    let server = Server::new(handle, addr);
    core.run(server.serve().expect("Can not listen to the address"))
        .unwrap();
}
