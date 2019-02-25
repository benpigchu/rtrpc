# rtrpc

An example used to demonstrate the concept of RPC. Built with the Tokio libraries.

## How to run

To run the demo, you need a stable channel Rust installation that support the Rust 2018 edition.
We recommend using the 1.32.0 version of rust.

To run the server, type `cargo run [IP Address]`

To run the example client, type `cd rtrpc_client&&cargo run --example main [IP Address]`

## Crates

There are three crates in this workspace. The `rtrpc_common` crate is for the common types and utils
between the server and the client. The `rtrpc_client` crate is for the client library. The main crate
is dor the server itself.