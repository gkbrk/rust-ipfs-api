# Rust IPFS API Client

[![Crates.io](https://img.shields.io/crates/v/ipfsapi.svg)](https://crates.io/crates/ipfsapi) [![Docs.rs](https://img.shields.io/badge/docs-ipfsapi-brightgreen.svg)](https://docs.rs/ipfsapi)

A client library for the IPFS API.

This library allows you to use the local IPFS daemon from Rust.

## Example

Here's an example that gets the contents of a file from IPFS and displays it.

```rust
let api = IpfsApi::new("127.0.0.1", 5001);

let bytes = api.cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u").unwrap();
let data = String::from_utf8(bytes.collect()).unwrap();

println!("{}", data);
```

The code gets the content of the IPFS hash and displays "Hello World".

## Implemented Functionality

* Getting file contents from IPFS (cat)
* Pubsub
* IPNS publish and resolve
* Object/Hash stats (size, etc.)

## Planned Functionality

* Pinning and unpinning files
