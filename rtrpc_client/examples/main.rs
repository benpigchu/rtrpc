use futures::Future;
use rtrpc_client::*;
use rtrpc_common::*;
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
    let start = "a";
    let end = "e";
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let addr = "127.0.0.1:12345".parse().unwrap();
    let client = Client::new(handle, addr);
    println!("request : {:?}", (&graph, &start, &end));
    let rpc = client
        .shortest_path(&graph, &start, &end)
        .map(|result| {
            println!("result : {:?}", result);
        })
        .map_err(|err| {
            println!("error : {:?}", err);
        });
    core.run(rpc).unwrap();
}