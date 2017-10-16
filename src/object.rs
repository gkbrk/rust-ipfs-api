use IpfsApi;

extern crate reqwest;
extern crate serde_json;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        JsonDecode(serde_json::Error);
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
pub struct ObjectStats {
    hash: String,
    cumulative_size: u64
}

impl IpfsApi {
    /// Get stats for an IPFS hash. It can be used to get the recursive size
    /// of a hash.
    pub fn object_stats(&self, hash: &str) -> Result<ObjectStats> {
        let url = format!("http://{}:{}/api/v0/object/stat?arg={}", self.server, self.port, hash);
        let resp = reqwest::get(&url)?;
        Ok(serde_json::from_reader(resp)?)
    }
}
