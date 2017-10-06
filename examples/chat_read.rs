extern crate ipfsapi;

use ipfsapi::IpfsApi;

fn main() {
    let api = IpfsApi::new("127.0.0.1", 5001);
    
    let messages = api.pubsub_subscribe("chat").unwrap();

    for message in messages {
        println!("{:?}", message);
    }
}
