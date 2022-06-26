extern crate ipfsapi;

use ipfsapi::IpfsApi;

fn main() {
    let api = IpfsApi::new("127.0.0.1", 5001);

    match api.version() {
        Ok(ver) => println!("{:?}", ver.version()),
        Err(_) => println!("Cannot connect to IPFS daemon"),
    }
}
