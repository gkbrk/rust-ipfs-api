use IpfsApi;

use reqwest;
use serde_json;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        JsonDecode(serde_json::Error);
    }
}

#[derive(Deserialize, Debug, PartialEq, Hash)]
#[serde(rename_all="PascalCase")]
pub struct ObjectStats {
    pub hash: String,
    pub num_links: u64,
    pub block_size: u64,
    pub links_size: u64,
    pub data_size: u64,
    pub cumulative_size: u64
}

impl IpfsApi {
    /// Get stats for an IPFS hash. It can be used to get the recursive size
    /// of a hash.
    pub fn object_stats(&self, hash: &str) -> Result<ObjectStats> {
        let url = format!("http://{}:{}/api/v0/object/stat?arg={}", self.server, self.port, hash);
        let resp = reqwest::get(&url)?.error_for_status()?;
        Ok(serde_json::from_reader(resp)?)
    }
}


#[cfg(test)]
mod tests {
    use IpfsApi;
    use super::*;
    #[test]
    fn test_object_stats() {
        let api = IpfsApi::new("127.0.0.1", 5001);
        // Hello world object
        let stats = api.object_stats("QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u").unwrap();
        let desired = ObjectStats {
            hash: "QmWATWQ7fVPP2EFGu71UkfnqhYXDYH566qy47CnJDgvs8u".to_string(),
            num_links: 0,
            block_size: 20,
            links_size: 2,
            data_size: 18,
            cumulative_size: 20,
        };

        assert_eq!(stats, desired);
    }
}
