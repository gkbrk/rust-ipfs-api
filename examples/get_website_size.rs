extern crate ipfsapi;

use ipfsapi::IpfsApi;

fn main() {
    let api = IpfsApi::new("127.0.0.1", 5001);
    let hash = api.name_resolve("gkbrk.com").unwrap();
    let website_stats = api.object_stats(&hash[6..]);

    println!("{:?}", website_stats);
}
