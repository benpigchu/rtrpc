use futures::Future;
use rtrpc_client::*;
use rtrpc_common::*;
use std::env::args;
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
    let argvec = args().collect::<Vec<_>>();
    let arg1 = match argvec.len() {
        1=>panic!("IP address not provided"),
        _=>&argvec[1]
    };
    let addr = arg1.parse().unwrap();
    let client = Client::new(handle, addr);
    let mut request=|graph,start,end|{
        println!("request : {:?}", (graph, start, end));
        let rpc = client
            .shortest_path(graph, start, end)
            .map(|result| {
                println!("result : {:?}", result);
            })
            .map_err(|err| {
                println!("error : {:?}", err);
            });
        core.run(rpc).unwrap();
    };
    request(&graph,"a","e");
    request(&graph,"f","h");
    request(&graph,"a","i");
}
