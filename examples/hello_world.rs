extern crate ipfsapi;

use ipfsapi::IpfsApi;

fn main() {
    let api = IpfsApi::new("127.0.0.1", 5001);
    
    let bytes = api.cat("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u").unwrap();
    let data = String::from_utf8(bytes.collect()).unwrap();

    println!("{}", data);
}
