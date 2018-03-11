extern crate ipfsapi;

use ipfsapi::IpfsApi;

fn main() {
    let api = IpfsApi::new("127.0.0.1", 5001);
    
    let logs = api.log_tail().unwrap();

    for log_line in logs {
        println!("{:?}", log_line);
    }
}
