mod core;
mod server;
use std::env::args;
use server::*;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let argvec = args().collect::<Vec<_>>();
    let arg1 = match argvec.len() {
        1=>panic!("IP address not provided"),
        _=>&argvec[1]
    };
    let addr = arg1.parse().expect("Invalid IP address");
    let server = Server::new(handle, addr);
    core.run(server.serve().expect("Can not listen to the address"))
        .unwrap();
}
